use monist_comb::ir::Comb;

/// Stratified Yoneda Lemma traversals.
/// Nat(C(U,-), F) \cong T(F(U)) natively on the hardware stack.
/// Ensures the universal set evaluates itself correctly bounded.
pub fn stratified_yoneda(f_u: Comb) -> Comb {
    // The Stratified Yoneda Lemma states that the natural transformations
    // from the representable functor C(U, -) to a functor F
    // are naturally isomorphic to T(F(U)).
    //
    // Thus, this traversal takes `F(U)` and applies the T operator to it.
    Comb::T.app(f_u)
}
