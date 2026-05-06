#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Comb {
    S,
    K,
    I,
    B,
    C,
    T,
    Terminal(String),
    Limit(usize, String, Box<Comb>),
    Var(String),
    App(Box<Comb>, Box<Comb>),

    // Set operations
    Eq,
    Mem,

    // Logic operations
    Neg,
    Conj,
    Disj,
    Impl,
    Forall,
}

impl Comb {
    pub fn app(self, other: Comb) -> Comb {
        Comb::App(Box::new(self), Box::new(other))
    }
}

impl Comb {
    pub fn contains_var(&self, v: &str) -> bool {
        match self {
            Comb::Var(name) => name == v,
            Comb::App(left, right) => left.contains_var(v) || right.contains_var(v),
            Comb::Limit(_, _, inner) => inner.contains_var(v),
            _ => false,
        }
    }

    pub fn abstract_var(self, var: &str) -> Comb {
        if !self.contains_var(var) {
            return Comb::K.app(self);
        }

        match self {
            Comb::Var(name) if name == var => Comb::I,
            Comb::App(left, right) => {
                let left_contains = left.contains_var(var);
                let right_contains = right.contains_var(var);

                if !left_contains && right_contains {
                    if let Comb::Var(name) = &*right {
                        if name == var {
                            return *left;
                        }
                    }
                    Comb::B.app(*left).app(right.abstract_var(var))
                } else if left_contains && !right_contains {
                    Comb::C.app(left.abstract_var(var)).app(*right)
                } else {
                    Comb::S
                        .app(left.abstract_var(var))
                        .app(right.abstract_var(var))
                }
            }
            _ => Comb::K.app(self), // Unreachable due to earlier check, but kept for completeness
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abstract_identity() {
        // \x. x -> I
        let x = Comb::Var("x".to_string());
        let abs = x.abstract_var("x");
        assert_eq!(abs, Comb::I);
    }

    #[test]
    fn test_abstract_const() {
        // \x. y -> K y
        let y = Comb::Var("y".to_string());
        let abs = y.clone().abstract_var("x");
        assert_eq!(abs, Comb::K.app(y));
    }

    #[test]
    fn test_abstract_app_s() {
        // \x. x x -> S I I
        let x = Comb::Var("x".to_string());
        let app = x.clone().app(x.clone());
        let abs = app.abstract_var("x");
        assert_eq!(abs, Comb::S.app(Comb::I).app(Comb::I));
    }

    #[test]
    fn test_abstract_app_b() {
        // \x. y x -> B y I (or similar based on optimizations)
        let x = Comb::Var("x".to_string());
        let y = Comb::Var("y".to_string());
        let app = y.clone().app(x.clone());
        let _abs = app.abstract_var("x");
        // Due to optimization: left=y (doesn't contain x), right=x (is exactly x)
        // It returns left, which is y (eta reduction).
        // Let's test a slightly more complex one: \x. y (z x)
        let z = Comb::Var("z".to_string());
        let app2 = y.clone().app(z.clone().app(x.clone()));
        let abs2 = app2.abstract_var("x");
        // left=y, right=z x
        // left doesn't contain x, right contains x
        // B y (\x. z x) -> B y z
        assert_eq!(abs2, Comb::B.app(y).app(z));
    }

    #[test]
    fn test_abstract_app_c() {
        // \x. (x y) z
        let x = Comb::Var("x".to_string());
        let y = Comb::Var("y".to_string());
        let z = Comb::Var("z".to_string());
        let app = x.clone().app(y.clone()).app(z.clone());
        let abs = app.abstract_var("x");
        // left = x y (contains x)
        // right = z (doesn't contain x)
        // C (\x. x y) z -> C ((\x. x) y) z -> wait, \x. x y -> left contains, right doesn't -> C (\x. x y) z -> C ((\x.x) y? no.
        // \x. x y -> right=y, left=x -> left contains, right doesn't -> C (\x. x) y -> C I y.
        // So \x. (x y) z -> left = x y, right = z -> C (\x. x y) z -> C (C I y) z
        assert_eq!(abs, Comb::C.app(Comb::C.app(Comb::I).app(y)).app(z));
    }
}
