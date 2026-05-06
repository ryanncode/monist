use super::encodings::{succ, v, y_comb};
use crate::ir::Comb;

// T-injections for establishing topological friction boundaries
pub fn t_inject(comb: Comb) -> Comb {
    Comb::T.app(comb)
}

// Cardinal summation (choice-free, using disjoint union logic)
// card_add = \m n f x. m f (n f x)
pub fn card_add() -> Comb {
    v("m")
        .app(v("f"))
        .app(v("n").app(v("f")).app(v("x")))
        .abstract_var("x")
        .abstract_var("f")
        .abstract_var("n")
        .abstract_var("m")
}

// Aleph_0: Infinite stream generator via Y Combinator
pub fn aleph_0() -> Comb {
    // Y (\x. Succ x)
    let f = v("x");
    let inner = succ().app(f);
    let abs = inner.abstract_var("x");
    y_comb().app(abs)
}

// Bounded infinite stream anchoring Aleph_0 structurally
pub fn bounded_aleph_0() -> Comb {
    t_inject(aleph_0())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_t_inject_aleph_0() {
        let b_aleph = bounded_aleph_0();
        match b_aleph {
            Comb::App(left, _) => {
                assert_eq!(*left, Comb::T);
            }
            _ => panic!("Expected T-injection application"),
        }
    }
}
