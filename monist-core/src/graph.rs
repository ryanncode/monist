use crate::ast::{Atomic, Formula, FormulaArena, Var};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ScopedVar(pub Var, pub usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Constraint {
    pub v1: ScopedVar,
    pub v2: ScopedVar,
    pub weight: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Edge {
    pub source: ScopedVar,
    pub target: ScopedVar,
    pub weight: i32,
}

impl From<Constraint> for Edge {
    fn from(c: Constraint) -> Self {
        Edge {
            source: c.v1,
            target: c.v2,
            weight: c.weight,
        }
    }
}

pub fn extract_constraints_aux(
    arena: &FormulaArena,
    formula_idx: usize,
    depth: usize,
) -> Vec<Constraint> {
    let mut constraints = Vec::new();

    let formula = match arena.get(formula_idx) {
        Some(f) => f,
        None => return constraints,
    };

    match formula {
        Formula::Atom(atomic) => match atomic {
            Atomic::Eq(x, y) => {
                let sx = ScopedVar(x.clone(), depth);
                let sy = ScopedVar(y.clone(), depth);
                constraints.push(Constraint { v1: sx.clone(), v2: sy.clone(), weight: 0 });
                constraints.push(Constraint { v1: sy, v2: sx, weight: 0 });
            }
            Atomic::Mem(x, y) => {
                let sx = ScopedVar(x.clone(), depth);
                let sy = ScopedVar(y.clone(), depth);
                constraints.push(Constraint { v1: sx.clone(), v2: sy.clone(), weight: 1 });
                constraints.push(Constraint { v1: sy, v2: sx, weight: -1 });
            }
            _ => {}
        },
        Formula::Neg(f_idx) => {
            constraints.extend(extract_constraints_aux(arena, *f_idx, depth));
        }
        Formula::Conj(f1_idx, f2_idx) => {
            constraints.extend(extract_constraints_aux(arena, *f1_idx, depth));
            constraints.extend(extract_constraints_aux(arena, *f2_idx, depth));
        }
        Formula::Disj(f1_idx, f2_idx) => {
            constraints.extend(extract_constraints_aux(arena, *f1_idx, depth));
            constraints.extend(extract_constraints_aux(arena, *f2_idx, depth));
        }
        Formula::Impl(f1_idx, f2_idx) => {
            constraints.extend(extract_constraints_aux(arena, *f1_idx, depth));
            constraints.extend(extract_constraints_aux(arena, *f2_idx, depth));
        }
        Formula::Univ(_, _, f_idx) => {
            constraints.extend(extract_constraints_aux(arena, *f_idx, depth + 1));
        }
        Formula::Comp(_, _, f_idx) => {
            constraints.extend(extract_constraints_aux(arena, *f_idx, depth + 1));
        }
    }
    constraints
}
