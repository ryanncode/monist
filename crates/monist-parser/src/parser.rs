use crate::lexer::{Lexer, Token};
use monist_core::ast::{Atomic, Formula, FormulaArena, Var};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    arena: &'a mut FormulaArena,
    bound_vars: Vec<String>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str, arena: &'a mut FormulaArena) -> Self {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.next_token();
        Self {
            lexer,
            current_token,
            arena,
            bound_vars: Vec::new(),
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
        self.parse_impl()
    }

    fn parse_impl(&mut self) -> usize {
        let left = self.parse_or();
        if self.match_token(Token::Impl) {
            let right = self.parse_impl();
            self.arena.add(Formula::Impl(left, right))
        } else {
            left
        }
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
        if self.match_token(Token::Not) {
            let inner = self.parse_unary();
            self.arena.add(Formula::Neg(inner))
        } else if self.match_token(Token::Forall) {
            if let Token::Ident(var) = self.current_token.clone() {
                self.advance();
                self.match_token(Token::Dot);
                self.bound_vars.push(var.clone());
                let inner = self.parse_formula();
                self.bound_vars.pop();
                self.arena.add(Formula::Univ(0, var, inner)) // dummy binder level 0 for now
            } else {
                panic!("Expected identifier after forall");
            }
        } else {
            self.parse_primary()
        }
    }

    fn parse_primary(&mut self) -> usize {
        if self.match_token(Token::LParen) {
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
            // Atomic: x = y or x in y
            let v1 = self.parse_var();
            if self.match_token(Token::Eq) {
                let v2 = self.parse_var();
                self.arena.add(Formula::Atom(Atomic::Eq(v1, v2)))
            } else if self.match_token(Token::In) {
                let v2 = self.parse_var();
                self.arena.add(Formula::Atom(Atomic::Mem(v1, v2)))
            } else {
                panic!("Expected = or in after variable");
            }
        }
    }
}
