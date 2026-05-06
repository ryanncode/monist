use super::encodings::{cons, v, y_comb, zero};
use crate::ir::Comb;

// Quine atom: \Omega = {\Omega}
// Represented via Y Combinator and List structures
// Omega = Y (\x. Cons x Nil)  -- where Nil is zero
pub fn quine_atom() -> Comb {
    let f = v("x");
    let inner = cons().app(f).app(zero());
    let abs = inner.abstract_var("x");
    y_comb().app(abs)
}

// Bounded quine atom (intercepts infinite recursion with Topologically-Guided limits)
pub fn bounded_quine_atom(limit: usize) -> Comb {
    Comb::Limit(
        limit,
        "K_ITERATION_HALT".to_string(),
        Box::new(quine_atom()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quine_bounded() {
        let bq = bounded_quine_atom(42);
        match bq {
            Comb::Limit(limit, halt_str, inner) => {
                assert_eq!(limit, 42);
                assert_eq!(halt_str, "K_ITERATION_HALT");
                assert_eq!(*inner, quine_atom());
            }
            _ => panic!("Expected bounded limit"),
        }
    }
}
