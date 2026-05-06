use super::encodings::{not_comb, v};
use crate::ir::Comb;

// Holographic data indexing exploits the absolute complement (V \ A)
// within a closed syntactic monist universe. This builds an "exclusion-first" gate.
pub fn exclusion_gate(target: Comb) -> Comb {
    // \x. Not (Eq x target)
    let eq = Comb::Eq.app(v("x")).app(target);
    not_comb().app(eq).abstract_var("x")
}

// Holographic search that applies the exclusion gate to immediately
// filter non-matching topological paths in O(1) time conceptually.
pub fn holographic_search() -> Comb {
    // \swarm target. swarm (exclusion_gate target)
    // We assume the swarm structure is capable of applying a filter gate.
    v("swarm")
        .app(exclusion_gate(v("target")))
        .abstract_var("target")
        .abstract_var("swarm")
}
