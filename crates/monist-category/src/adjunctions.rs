use monist_comb::ir::Comb;

/// T-Relative Cartesian Closure evaluation maps.
/// Constructs a modified evaluation map ev'_{A,B}: TA x (A => B) -> TB.
/// This absorbs topological friction to simulate Cartesian closure in a monist universe.
pub fn ev_prime() -> Comb {
    // We implement `ev_prime` as a combinator structure.
    // T-Relative evaluation involves the T-operator applied to absorb the shift.
    // ev'_{A,B}(t_a, f) = f(a) shifted by T.
    // Using combinators, if the arguments are `ta` (which is T a) and `f`,
    // the application is T(f a), but since `f` takes `A`, we need to extract `a` or apply it correctly.
    // We construct the composition directly.
    Comb::S.app(Comb::K.app(Comb::T)).app(Comb::I)
}
