#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Var {
    Free(String),
    Bound(usize),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Atomic {
    Eq(Var, Var),
    Mem(Var, Var),
    QPair,
    QProj1,
    QProj2,
    App,
    Lam,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Formula {
    Atom(Atomic),
    Neg(Box<Formula>),
    Conj(Box<Formula>, Box<Formula>),
    Disj(Box<Formula>, Box<Formula>),
    Impl(Box<Formula>, Box<Formula>),
    Univ(usize, String, Box<Formula>),
    Comp(usize, String, Box<Formula>),
}
