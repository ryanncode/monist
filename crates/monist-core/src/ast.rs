#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Var {
    Free(String),
    Bound(usize),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Atomic {
    Eq(Var, Var),
    Mem(Var, Var),
    QPair,
    QProj1,
    QProj2,
    App,
    Lam,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Formula {
    Atom(Atomic),
    Neg(usize),
    Conj(usize, usize),
    Disj(usize, usize),
    Impl(usize, usize),
    Univ(usize, String, usize),
    Exist(usize, String, usize),
    Comp(usize, String, usize),
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct FormulaArena {
    nodes: Vec<Formula>,
}

impl FormulaArena {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, formula: Formula) -> usize {
        let index = self.nodes.len();
        self.nodes.push(formula);
        index
    }

    pub fn get(&self, index: usize) -> Option<&Formula> {
        self.nodes.get(index)
    }
}
