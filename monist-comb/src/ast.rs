#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Comb {
    K,
    S,
    I,
    U,
    Var(String),
    App(Box<Comb>, Box<Comb>),
    TInject(Box<Comb>),
    LazyThunk(Box<Comb>),
    Terminal(String),
}
