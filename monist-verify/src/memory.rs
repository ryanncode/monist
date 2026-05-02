use std::mem::MaybeUninit;

/// A wrapper around uninitialized memory that explicitly stabilizes it to a semantic zero.
/// This prevents model checkers (like RustMC) from stalling on `undef` LLVM values by
/// enforcing explicit zero-initialization before the read.
pub struct StabilizedArena<T> {
    data: Vec<MaybeUninit<T>>,
}

impl<T> StabilizedArena<T> {
    /// Creates a new uninitialized arena with the given capacity.
    pub fn new(capacity: usize) -> Self {
        let mut data = Vec::with_capacity(capacity);
        // We model RustMC's behavior by explicitly filling allocations with a known state,
        // specifically zeroing out the bytes. This transforms uninitialized memory
        // into a safe semantic zero for the model checker.
        unsafe {
            let ptr = data.as_mut_ptr() as *mut u8;
            std::ptr::write_bytes(ptr, 0, capacity * std::mem::size_of::<T>());
            data.set_len(capacity);
        }
        Self { data }
    }

    /// Read an element from the arena. The explicit zeroing guarantees no `undef` read.
    pub fn read(&self, index: usize) -> Option<&T> {
        if index < self.data.len() {
            // Safety: The memory was explicitly zeroed during allocation,
            // avoiding LLVM undef behavior during model checking.
            unsafe { Some(self.data[index].assume_init_ref()) }
        } else {
            None
        }
    }

    /// Write an element into the arena.
    pub fn write(&mut self, index: usize, value: T) {
        if index < self.data.len() {
            self.data[index].write(value);
        }
    }
}
