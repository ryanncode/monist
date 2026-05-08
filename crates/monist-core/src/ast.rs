/// Represents a variable in the logical formula.
/// Variables can either be freely named (`Free`) or bound to a De Bruijn index (`Bound`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Var {
    /// A free variable represented by its name.
    Free(String),
    /// A bound variable represented by its De Bruijn index.
    Bound(usize),
}

/// Represents atomic propositions in the logic system.
/// This includes fundamental set-theoretic and topological relations.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Atomic {
    /// Equality between two variables (`v1 = v2`).
    Eq(Var, Var),
    /// Set membership between two variables (`v1 ∈ v2`).
    Mem(Var, Var),
    /// Less-than relation, used for stratifications (`v1 < v2`).
    Lt(Var, Var),
    /// Quine pair constructor.
    QPair,
    /// Quine pair first projection.
    QProj1,
    /// Quine pair second projection.
    QProj2,
    /// Application for interaction nets.
    App,
    /// Lambda abstraction for interaction nets.
    Lam,
}

/// Represents a recursive logical formula in the Abstract Syntax Tree (AST).
/// 
/// The structure forms a directed acyclic graph when mapped to `FormulaArena`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Formula {
    /// An atomic proposition.
    Atom(Atomic),
    /// Logical negation of an inner formula.
    Neg(usize),
    /// Logical conjunction (AND) of two formulas.
    Conj(usize, usize),
    /// Logical disjunction (OR) of two formulas.
    Disj(usize, usize),
    /// Logical implication (=>) between two formulas.
    Impl(usize, usize),
    /// Universal quantification (∀) over a variable name in an inner formula.
    Univ(usize, String, usize),
    /// Existential quantification (∃) over a variable name in an inner formula.
    Exist(usize, String, usize),
    /// Set comprehension ({x | P(x)}), defining a new set based on a predicate.
    Comp(usize, String, usize),
}

/// An arena-based allocator for managing the `Formula` abstract syntax tree.
/// 
/// By storing `Formula` nodes in a flat vector and referencing them by `usize` indices, 
/// the system builds a Graph Reduction representation of the AST suitable for the 
/// CPU Geometry Layer. This approach ensures locality and prevents deeply recursive structures.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct FormulaArena {
    nodes: Vec<Formula>,
}

impl FormulaArena {
    /// Creates a new, empty `FormulaArena`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a new `Formula` node to the arena and returns its index.
    /// 
    /// This index acts as a pointer to the formula and can be used in parent nodes.
    pub fn add(&mut self, formula: Formula) -> usize {
        let index = self.nodes.len();
        self.nodes.push(formula);
        index
    }

    /// Retrieves a reference to a `Formula` by its index in the arena.
    /// Returns `None` if the index is out of bounds.
    pub fn get(&self, index: usize) -> Option<&Formula> {
        self.nodes.get(index)
    }
}
