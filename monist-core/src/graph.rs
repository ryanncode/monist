use crate::ast::{Atomic, Formula, Var};

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

pub fn extract_constraints_aux(formula: &Formula, depth: usize) -> Vec<Constraint> {
    let mut constraints = Vec::new();
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
        Formula::Neg(f) => {
            constraints.extend(extract_constraints_aux(f, depth));
        }
        Formula::Conj(f1, f2) => {
            constraints.extend(extract_constraints_aux(f1, depth));
            constraints.extend(extract_constraints_aux(f2, depth));
        }
        Formula::Disj(f1, f2) => {
            constraints.extend(extract_constraints_aux(f1, depth));
            constraints.extend(extract_constraints_aux(f2, depth));
        }
        Formula::Impl(f1, f2) => {
            constraints.extend(extract_constraints_aux(f1, depth));
            constraints.extend(extract_constraints_aux(f2, depth));
        }
        Formula::Univ(_, _, f) => {
            constraints.extend(extract_constraints_aux(f, depth + 1));
        }
        Formula::Comp(_, _, f) => {
            constraints.extend(extract_constraints_aux(f, depth + 1));
        }
    }
    constraints
}
