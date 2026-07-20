use crate::lexer::{Lexer, Token};
use monist_core::ast::{Atomic, Formula, FormulaArena, Var};
use monist_core::budget::ResourceBudget;

use std::collections::HashMap;

pub struct Parser<'a> {
    macros: Option<&'a HashMap<String, (Vec<String>, usize)>>,
    lexer: Lexer<'a>,
    current_token: Token,
    arena: &'a mut FormulaArena,
    bound_vars: Vec<String>,
    budget: ResourceBudget,
    current_depth: usize,
    nodes_allocated: usize,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str, arena: &'a mut FormulaArena, budget: ResourceBudget) -> Self {
        Self::with_macros(input, arena, None, budget)
    }

    pub fn with_macros(
        input: &'a str,
        arena: &'a mut FormulaArena,
        macros: Option<&'a HashMap<String, (Vec<String>, usize)>>,
        budget: ResourceBudget,
    ) -> Self {
        if input.len() > budget.max_bytes {
            panic!("Input Too Large");
        }
        let mut lexer = Lexer::new(input);
        let current_token = lexer.next_token();
        Self {
            lexer,
            current_token,
            arena,
            bound_vars: Vec::new(),
            macros,
            budget,
            current_depth: 0,
            nodes_allocated: 0,
        }
    }

    fn check_budget(&mut self) {
        if self.current_depth > self.budget.max_depth {
            panic!("Nesting Limit Exceeded");
        }
        self.nodes_allocated += 1;
        if self.nodes_allocated > self.budget.max_ast_nodes {
            panic!("AST Node Limit Exceeded");
        }
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    fn match_token(&mut self, expected: Token) -> bool {
        if self.current_token == expected {
            self.advance();
            true
        } else {
            false
        }
    }

    fn parse_var(&mut self) -> Var {
        if let Token::Ident(name) = &self.current_token {
            let name_clone = name.clone();
            self.advance();
            if let Some(pos) = self.bound_vars.iter().rev().position(|v| v == &name_clone) {
                Var::Bound(pos)
            } else {
                Var::Free(name_clone)
            }
        } else {
            panic!("Expected identifier");
        }
    }

    pub fn parse_formula(&mut self) -> usize {
        self.current_depth += 1;
        self.check_budget();
        let res = self.parse_iff();
        self.current_depth -= 1;
        res
    }

    fn parse_iff(&mut self) -> usize {
        self.current_depth += 1;
        self.check_budget();
        let left = self.parse_impl();
        let res = if self.match_token(Token::Iff) {
            let right = self.parse_iff();
            let lr = self.arena.add(Formula::Impl(left, right));
            let rl = self.arena.add(Formula::Impl(right, left));
            self.arena.add(Formula::Conj(lr, rl))
        } else {
            left
        };
        self.current_depth -= 1;
        res
    }

    fn parse_impl(&mut self) -> usize {
        self.current_depth += 1;
        self.check_budget();
        let left = self.parse_or();
        let res = if self.match_token(Token::Impl) {
            let right = self.parse_impl();
            self.arena.add(Formula::Impl(left, right))
        } else {
            left
        };
        self.current_depth -= 1;
        res
    }

    fn parse_or(&mut self) -> usize {
        let mut left = self.parse_and();
        while self.match_token(Token::Or) {
            let right = self.parse_and();
            left = self.arena.add(Formula::Disj(left, right));
        }
        left
    }

    fn parse_and(&mut self) -> usize {
        let mut left = self.parse_unary();
        while self.match_token(Token::And) {
            let right = self.parse_unary();
            left = self.arena.add(Formula::Conj(left, right));
        }
        left
    }

    fn parse_unary(&mut self) -> usize {
        self.current_depth += 1;
        self.check_budget();
        let res = if self.match_token(Token::Not) {
            let inner = self.parse_unary();
            self.arena.add(Formula::Neg(inner))
        } else if self.match_token(Token::Forall) {
            let mut vars = Vec::new();
            while let Token::Ident(var) = self.current_token.clone() {
                vars.push(var);
                self.advance();
            }
            if vars.is_empty() {
                panic!("Expected identifier after forall");
            }
            self.match_token(Token::Dot);
            for var in &vars {
                self.bound_vars.push(var.clone());
            }
            let mut inner = self.parse_formula();
            for var in vars.iter().rev() {
                self.bound_vars.pop();
                inner = self.arena.add(Formula::Univ(0, var.clone(), inner));
            }
            inner
        } else if self.match_token(Token::Exists) {
            let mut vars = Vec::new();
            while let Token::Ident(var) = self.current_token.clone() {
                vars.push(var);
                self.advance();
            }
            if vars.is_empty() {
                panic!("Expected identifier after exists");
            }
            self.match_token(Token::Dot);
            for var in &vars {
                self.bound_vars.push(var.clone());
            }
            let mut inner = self.parse_formula();
            for var in vars.iter().rev() {
                self.bound_vars.pop();
                inner = self.arena.add(Formula::Exist(0, var.clone(), inner));
            }
            inner
        } else {
            self.parse_primary()
        };
        self.current_depth -= 1;
        res
    }

    fn parse_primary(&mut self) -> usize {
        self.current_depth += 1;
        self.check_budget();
        let res = if self.match_token(Token::LParen) {
            let inner = self.parse_formula();
            self.match_token(Token::RParen);
            inner
        } else if self.match_token(Token::LBrace) {
            // Comprehension: { x | P(x) }
            if let Token::Ident(var) = self.current_token.clone() {
                self.advance();
                self.match_token(Token::Bar);
                self.bound_vars.push(var.clone());
                let inner = self.parse_formula();
                self.bound_vars.pop();
                self.match_token(Token::RBrace);
                self.arena.add(Formula::Comp(0, var, inner))
            } else {
                panic!("Expected identifier in comprehension");
            }
        } else {
            // Atomic: x = y or x in y or macro
            let v1 = self.parse_var();
            if self.match_token(Token::Eq) {
                let v2 = self.parse_var();
                self.arena.add(Formula::Atom(Atomic::Eq(v1, v2)))
            } else if self.match_token(Token::In) {
                let v2 = self.parse_var();
                self.arena.add(Formula::Atom(Atomic::Mem(v1, v2)))
            } else if self.match_token(Token::LessThan) {
                let v2 = self.parse_var();
                self.arena.add(Formula::Atom(Atomic::Lt(v1, v2)))
            } else if self.match_token(Token::LParen) {
                if let Var::Free(name) = v1 {
                    let mut args = Vec::new();
                    if self.current_token != Token::RParen {
                        args.push(self.parse_var());
                        while self.match_token(Token::Comma) {
                            args.push(self.parse_var());
                        }
                    }
                    self.match_token(Token::RParen);
                    if let Some(macros) = self.macros {
                        if let Some((params, formula_idx)) = macros.get(&name) {
                            if params.len() == args.len() {
                                let expanded = self.expand_macro(*formula_idx, params, &args);
                                self.current_depth -= 1;
                                return expanded;
                            }
                        }
                    }
                    panic!("Macro {} not found or wrong arity", name);
                } else {
                    panic!("Expected macro name");
                }
            } else {
                self.arena.add(Formula::Atom(Atomic::Eq(v1.clone(), v1))) // fallback
            }
        };
        self.current_depth -= 1;
        res
    }

    fn expand_macro(&mut self, root: usize, params: &[String], args: &[Var]) -> usize {
        self.current_depth += 1;
        self.check_budget();
        let f = match self.arena.get(root) {
            Some(f) => f.clone(),
            None => {
                self.current_depth -= 1;
                return root;
            }
        };

        let map_var = |v: &Var| -> Var {
            match v {
                Var::Free(s) => {
                    if let Some(pos) = params.iter().position(|p| p == s) {
                        args[pos].clone()
                    } else {
                        v.clone()
                    }
                }
                _ => v.clone(),
            }
        };

        let res = match f {
            Formula::Atom(mut atomic) => {
                match &mut atomic {
                    Atomic::Eq(v1, v2) | Atomic::Mem(v1, v2) | Atomic::Lt(v1, v2) => {
                        *v1 = map_var(v1);
                        *v2 = map_var(v2);
                    }
                    _ => {}
                }
                self.arena.add(Formula::Atom(atomic))
            }
            Formula::Neg(i) => {
                let ni = self.expand_macro(i, params, args);
                self.arena.add(Formula::Neg(ni))
            }
            Formula::Conj(l, r) => {
                let nl = self.expand_macro(l, params, args);
                let nr = self.expand_macro(r, params, args);
                self.arena.add(Formula::Conj(nl, nr))
            }
            Formula::Disj(l, r) => {
                let nl = self.expand_macro(l, params, args);
                let nr = self.expand_macro(r, params, args);
                self.arena.add(Formula::Disj(nl, nr))
            }
            Formula::Impl(l, r) => {
                let nl = self.expand_macro(l, params, args);
                let nr = self.expand_macro(r, params, args);
                self.arena.add(Formula::Impl(nl, nr))
            }
            Formula::Univ(b, s, inner) => {
                let ninner = self.expand_macro(inner, params, args);
                self.arena.add(Formula::Univ(b, s, ninner))
            }
            Formula::Exist(b, s, inner) => {
                let ninner = self.expand_macro(inner, params, args);
                self.arena.add(Formula::Exist(b, s, ninner))
            }
            Formula::Comp(b, s, inner) => {
                let ninner = self.expand_macro(inner, params, args);
                self.arena.add(Formula::Comp(b, s, ninner))
            }
        };
        self.current_depth -= 1;
        res
    }
}
