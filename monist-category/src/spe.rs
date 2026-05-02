use monist_comb::ir::Comb;
use crate::adjunctions::ev_prime;
use crate::retractions::sc_cut;
use crate::yoneda::stratified_yoneda;

/// The Stratified Pseudo Elephant (SPE) Architecture.
/// Coordinates T-Relative Adjunctions, SC Retractions, and Yoneda Traversals.
pub struct SpeArchitecture;

impl SpeArchitecture {
    pub fn new() -> Self {
        Self
    }
    
    pub fn evaluate_adjunction(&self) -> Comb {
        ev_prime()
    }
    
    pub fn encapsulate_sc(&self, logic: Comb) -> Comb {
        sc_cut(logic)
    }
    
    pub fn apply_yoneda(&self, functor_applied: Comb) -> Comb {
        stratified_yoneda(functor_applied)
    }
}
