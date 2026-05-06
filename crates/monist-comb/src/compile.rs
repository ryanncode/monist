use crate::ir::Comb;
use monist_core::ast::{Atomic, Formula, FormulaArena, Var};
use std::collections::HashMap;

pub struct Compiler<'a> {
    arena: &'a FormulaArena,
    min_levels: HashMap<String, i32>,
}

impl<'a> Compiler<'a> {
    pub fn new(arena: &'a FormulaArena) -> Self {
        Self {
            arena,
            min_levels: HashMap::new(),
        }
    }

    pub fn compile(&mut self, root: usize) -> Comb {
        // Step 1: Extract constraints to compute execution limits
        let constraints = monist_core::graph::extract_constraints_aux(self.arena, root, 0, false);
        let graph = monist_core::graph::GraphArena::from_constraints(&constraints);
        let limits = monist_core::eval::ExecutionLimits::compute_for_graph(&graph);

        // Step 2: Compute min req levels for T-weaking
        self.min_levels.clear();
        self.compute_min_levels(root, 0, &mut Vec::new());

        // Step 3: Compile with env
        let compiled = self.compile_with_env(root, 0, &mut Vec::new());

        // Step 4: Map ExecutionLimits into the IR
        if let Some(lim) = limits {
            Comb::Limit(
                lim.max_k_iterations,
                lim.mcm.to_string(),
                Box::new(compiled),
            )
        } else {
            compiled
        }
    }

    fn compute_min_levels(&mut self, root: usize, expected_level: i32, env: &mut Vec<String>) {
        let formula = self.arena.get(root).expect("Invalid formula index");
        match formula {
            Formula::Atom(atomic) => match atomic {
                Atomic::Eq(v1, v2) => {
                    self.register_var_req(v1, expected_level, env);
                    self.register_var_req(v2, expected_level, env);
                }
                Atomic::Mem(v1, v2) => {
                    self.register_var_req(v1, expected_level - 1, env);
                    self.register_var_req(v2, expected_level, env);
                }
                _ => {}
            },
            Formula::Neg(inner) => self.compute_min_levels(*inner, expected_level, env),
            Formula::Conj(l, r) | Formula::Disj(l, r) => {
                self.compute_min_levels(*l, expected_level, env);
                self.compute_min_levels(*r, expected_level, env);
            }
            Formula::Impl(l, r) => {
                self.compute_min_levels(*l, expected_level - 1, env);
                self.compute_min_levels(*r, expected_level - 1, env);
            }
            Formula::Univ(_level, var_name, inner)
            | Formula::Exist(_level, var_name, inner)
            | Formula::Comp(_level, var_name, inner) => {
                env.push(var_name.clone());
                self.compute_min_levels(*inner, expected_level, env);
                env.pop();
            }
        }
    }

    fn register_var_req(&mut self, var: &Var, req: i32, env: &[String]) {
        let name = match var {
            Var::Free(name) => name.clone(),
            Var::Bound(idx) => env[env.len() - 1 - *idx].clone(),
        };
        let current_min = self.min_levels.entry(name).or_insert(req);
        if req < *current_min {
            *current_min = req;
        }
    }

    fn compile_with_env(&self, root: usize, expected_level: i32, env: &mut Vec<String>) -> Comb {
        let formula = self.arena.get(root).expect("Invalid formula index");
        match formula {
            Formula::Atom(atomic) => self.compile_atomic(atomic, expected_level, env),
            Formula::Neg(inner) => {
                Comb::Neg.app(self.compile_with_env(*inner, expected_level, env))
            }
            Formula::Conj(l, r) => Comb::Conj
                .app(self.compile_with_env(*l, expected_level, env))
                .app(self.compile_with_env(*r, expected_level, env)),
            Formula::Disj(l, r) => Comb::Disj
                .app(self.compile_with_env(*l, expected_level, env))
                .app(self.compile_with_env(*r, expected_level, env)),
            Formula::Impl(l, r) => Comb::Impl
                .app(self.compile_with_env(*l, expected_level - 1, env))
                .app(self.compile_with_env(*r, expected_level - 1, env)),
            Formula::Univ(_level, var_name, inner) => {
                env.push(var_name.clone());
                let inner_comb = self.compile_with_env(*inner, expected_level, env);
                env.pop();
                let abstracted = inner_comb.abstract_var(var_name);
                Comb::Forall.app(abstracted)
            }
            Formula::Exist(_level, var_name, inner) => {
                env.push(var_name.clone());
                let inner_comb = self.compile_with_env(*inner, expected_level, env);
                env.pop();
                let abstracted = inner_comb.abstract_var(var_name);
                Comb::Var("Exists".to_string()).app(abstracted)
            }
            Formula::Comp(_level, var_name, inner) => {
                env.push(var_name.clone());
                let inner_comb = self.compile_with_env(*inner, expected_level, env);
                env.pop();
                // Comprehension {x | P(x)} translates to an abstraction over x of P(x)
                inner_comb.abstract_var(var_name)
            }
        }
    }

    fn compile_atomic(&self, atomic: &Atomic, expected_level: i32, env: &[String]) -> Comb {
        match atomic {
            Atomic::Eq(v1, v2) => {
                let cv1 = self.compile_var(v1, expected_level, env);
                let cv2 = self.compile_var(v2, expected_level, env);
                Comb::Eq.app(cv1).app(cv2)
            }
            Atomic::Mem(v1, v2) => {
                let cv1 = self.compile_var(v1, expected_level - 1, env);
                let cv2 = self.compile_var(v2, expected_level, env);
                Comb::Mem.app(cv1).app(cv2)
            }
            Atomic::QPair => Comb::Var("QPair".to_string()),
            Atomic::QProj1 => Comb::Var("QProj1".to_string()),
            Atomic::QProj2 => Comb::Var("QProj2".to_string()),
            Atomic::App => Comb::Var("App".to_string()),
            Atomic::Lam => Comb::Var("Lam".to_string()),
        }
    }

    fn compile_var(&self, var: &Var, expected_level: i32, env: &[String]) -> Comb {
        let name = match var {
            Var::Free(n) => n.clone(),
            Var::Bound(idx) => env[env.len() - 1 - *idx].clone(),
        };

        let mut comb = Comb::Var(name.clone());

        // T-Weaking Algorithm
        // Inject T operators if the required expected_level is higher than the base min_level
        if let Some(&min_lvl) = self.min_levels.get(&name) {
            if expected_level > min_lvl {
                let diff = expected_level - min_lvl;
                for _ in 0..diff {
                    comb = Comb::T.app(comb);
                }
            }
        }

        comb
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use monist_core::ast::{Atomic, Formula, Var};

    #[test]
    fn test_compile_formula() {
        let mut arena = FormulaArena::new();
        // Compile a simple formula: {x | x in y} -> Should translate to an abstraction over x
        let inner = arena.add(Formula::Atom(Atomic::Mem(
            Var::Free("x".to_string()),
            Var::Free("y".to_string()),
        )));
        let comp = arena.add(Formula::Comp(0, "x".to_string(), inner));

        let mut compiler = Compiler::new(&arena);
        let result = compiler.compile(comp);

        // Check that Limits wrapper is applied correctly and C Mem y is produced.
        match result {
            Comb::Limit(_, _, inner) => {
                assert_eq!(
                    *inner,
                    Comb::C.app(Comb::Mem).app(Comb::Var("y".to_string()))
                );
            }
            _ => panic!("Expected Limit wrapper"),
        }
    }

    #[test]
    fn test_t_weaking() {
        let mut arena = FormulaArena::new();
        // Formula: x = y => x in y
        // root level = 0
        // Left side (x = y) at level -1: x expected at -1, y expected at -1
        // Right side (x in y) at level -1: x expected at -2, y expected at -1
        // x min level = -2.
        // Left side x needs level -1, so it gets 1 T-operator: T x
        // Right side x needs level -2, so 0 T-operators: x

        let eq = arena.add(Formula::Atom(Atomic::Eq(
            Var::Free("x".to_string()),
            Var::Free("y".to_string()),
        )));
        let mem = arena.add(Formula::Atom(Atomic::Mem(
            Var::Free("x".to_string()),
            Var::Free("y".to_string()),
        )));
        let impl_form = arena.add(Formula::Impl(eq, mem));

        let mut compiler = Compiler::new(&arena);
        let result = compiler.compile(impl_form);

        match result {
            Comb::Limit(_, _, inner) => {
                let Comb::App(l1, r1) = *inner else { panic!() };
                let Comb::App(impl_op, left_eq) = *l1 else {
                    panic!()
                };
                assert_eq!(*impl_op, Comb::Impl);

                // Left side: Eq (T x) y
                assert_eq!(
                    *left_eq,
                    Comb::Eq
                        .app(Comb::T.app(Comb::Var("x".to_string())))
                        .app(Comb::Var("y".to_string()))
                );

                // Right side: Mem x y
                assert_eq!(
                    *r1,
                    Comb::Mem
                        .app(Comb::Var("x".to_string()))
                        .app(Comb::Var("y".to_string()))
                );
            }
            _ => panic!("Expected Limit wrapper"),
        }
    }
}
