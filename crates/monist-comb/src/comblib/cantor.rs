use super::encodings::v;
use crate::ir::Comb;

/// Cantor's Theorem and Power Set Cardinal Shift in New Foundations.
///
/// In ZFC, Cantor's theorem states |A| < |P(A)| for all sets A.
/// If applied naively to the Universal Set V, it would yield |V| < |P(V)| <= |V| (Contradiction).
///
/// In NF, Cantor's theorem holds in the modified form:
/// |P_1(A)| < |P(A)|, where P_1(A) = {{x} | x \in A} is the set of singletons of A.
/// For strongly Cantorian sets, |A| = |P_1(A)|, so |A| < |P(A)|.
/// For the Universal Set V, |P_1(V)| = |T(V)| < |P(V)| <= |V|.

/// Singleton map: x -> {x}
/// Note that this operation increases the stratum / typestate level by +1 (T operator).
pub fn singleton_map() -> Comb {
    // \x y. y = x
    Comb::Eq.app(v("y")).app(v("x")).abstract_var("y").abstract_var("x")
}

/// Power set predicate: P(A) = {s | s \subseteq A} = {s | \forall x. x \in s -> x \in A}
pub fn power_set(a: Comb) -> Comb {
    // \s. Forall (\x. (s x) -> (A x))
    let x_in_s = v("s").app(v("x"));
    let x_in_a = a.app(v("x"));
    let impl_term = Comb::Impl.app(x_in_s).app(x_in_a);
    let forall_term = Comb::Forall.app(impl_term.abstract_var("x"));
    forall_term.abstract_var("s")
}

/// Bounded Cantor singleton map with T-operator shift
pub fn t_shifted_singleton_map() -> Comb {
    Comb::T.app(singleton_map())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singleton_map_structure() {
        let s_map = singleton_map();
        assert!(matches!(s_map, Comb::App(_, _)));
    }

    #[test]
    fn test_power_set_combinator() {
        let u_set = Comb::K.app(Comb::I);
        let p_u = power_set(u_set);
        assert!(matches!(p_u, Comb::App(_, _)));
    }

    #[test]
    fn test_t_shifted_singleton() {
        let t_s = t_shifted_singleton_map();
        match t_s {
            Comb::App(left, _) => assert_eq!(*left, Comb::T),
            _ => panic!("Expected T application on singleton map"),
        }
    }
}

