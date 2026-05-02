use std::sync::atomic::{AtomicU64, Ordering};

/// A concurrent wrapper for Interaction Net node collisions.
/// Nodes are represented as 64-bit words, optimized for the Monist Engine's
/// heavily parallelized, arena-allocated graphs.
pub struct AtomicNode {
    /// The physical state of the node compressed into a 64-bit word.
    state: AtomicU64,
}

impl AtomicNode {
    pub fn new(initial_state: u64) -> Self {
        Self {
            state: AtomicU64::new(initial_state),
        }
    }

    /// Executes a lock-free Interaction Net node collision.
    /// This atomic exchange swaps the new interacting node state with the current one.
    /// A stateless model checker like RustMC will explore all thread interleavings
    /// of this relaxed atomic operation to verify thread safety and absence of data races.
    pub fn collide(&self, incoming_node: u64) -> u64 {
        // We use Ordering::AcqRel to ensure memory coherence across threads during the collision.
        // RustMC extends GenMC to verify that this lock-free exchange does not induce data races
        // in the heavily parallelized arena.
        self.state.swap(incoming_node, Ordering::AcqRel)
    }

    /// Reads the current state of the node.
    pub fn load(&self) -> u64 {
        self.state.load(Ordering::Acquire)
    }

    /// Unconditionally stores a new state.
    pub fn store(&self, value: u64) {
        self.state.store(value, Ordering::Release)
    }
}
