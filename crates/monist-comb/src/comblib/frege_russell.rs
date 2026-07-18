use crate::ir::Comb;
use crate::compile::Compiler;
use monist_core::ast::{Atomic, Formula, FormulaArena, Var};
use std::collections::HashMap;

/// Compiles a static AST formula into a Combinator representation.
fn compile_formula(arena: &FormulaArena, root: usize) -> Comb {
    let mut compiler = Compiler::new(arena);
    compiler.compile(root)
}

/// A Frege-Russell numeral `zero` is the set of all empty sets.
/// We represent it natively using NF topological set comprehension: { y | \forall x (x \notin y) }
pub fn frege_zero() -> Comb {
    let mut arena = FormulaArena::new();
    
    let x_in_y = arena.add(Formula::Atom(Atomic::Mem(Var::Free("x".to_string()), Var::Free("y".to_string()))));
    let x_notin_y = arena.add(Formula::Neg(x_in_y));
    let y_is_empty = arena.add(Formula::Univ(0, "x".to_string(), x_notin_y));
    let zero_comp = arena.add(Formula::Comp(0, "y".to_string(), y_is_empty));
    
    compile_formula(&arena, zero_comp)
}

/// Successor function: succ(n) = { x | \exists u \in n, \exists c \notin u (x = u \cup {c}) }
/// Expressed logically: \n . { x | \exists u (u \in n \wedge \exists c (c \notin u \wedge \forall z (z \in x <-> z \in u \vee z = c))) }
pub fn frege_succ() -> Comb {
    let mut arena = FormulaArena::new();
    
    let z_eq_c = arena.add(Formula::Atom(Atomic::Eq(Var::Free("z".to_string()), Var::Free("c".to_string()))));
    let z_in_u = arena.add(Formula::Atom(Atomic::Mem(Var::Free("z".to_string()), Var::Free("u".to_string()))));
    let z_in_u_or_c = arena.add(Formula::Disj(z_in_u, z_eq_c));
    let z_in_x = arena.add(Formula::Atom(Atomic::Mem(Var::Free("z".to_string()), Var::Free("x".to_string()))));
    
    let a_to_b = arena.add(Formula::Impl(z_in_x, z_in_u_or_c));
    let b_to_a = arena.add(Formula::Impl(z_in_u_or_c, z_in_x));
    let iff = arena.add(Formula::Conj(a_to_b, b_to_a));
    
    let forall_z = arena.add(Formula::Univ(0, "z".to_string(), iff));
    
    let c_in_u = arena.add(Formula::Atom(Atomic::Mem(Var::Free("c".to_string()), Var::Free("u".to_string()))));
    let c_notin_u = arena.add(Formula::Neg(c_in_u));
    let c_cond = arena.add(Formula::Conj(c_notin_u, forall_z));
    let exists_c = arena.add(Formula::Exist(0, "c".to_string(), c_cond));
    
    let u_in_n = arena.add(Formula::Atom(Atomic::Mem(Var::Free("u".to_string()), Var::Free("n".to_string()))));
    let u_cond = arena.add(Formula::Conj(u_in_n, exists_c));
    let exists_u = arena.add(Formula::Exist(0, "u".to_string(), u_cond));
    
    let succ_comp = arena.add(Formula::Comp(0, "x".to_string(), exists_u));
    let compiled = compile_formula(&arena, succ_comp);
    
    compiled.abstract_var("n")
}
