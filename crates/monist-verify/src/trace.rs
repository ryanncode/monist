/// Mock interface simulating RustMC's tracking of LLVM memory intrinsics over untyped memory arenas.
/// This translates operations into explicitly typed 64-bit word sequences for the HVM2 backend.
pub struct IntrinsicTracker {
    pub ops: Vec<MemoryOp>,
}

pub enum MemoryOp {
    MemCpyOpt { src: usize, dest: usize, len: usize },
    IpSccp { ptr: usize, value: u64 },
}

impl IntrinsicTracker {
    pub fn new() -> Self {
        Self { ops: Vec::new() }
    }

    /// Simulates intercepting `memcpyopt` pass to promote mixed-size accesses into 64-bit words.
    /// This enables explicit physical memory layout tracking of the combinator net by the model checker.
    pub fn track_memcpyopt(&mut self, src: *const u8, dest: *mut u8, len: usize) {
        self.ops.push(MemoryOp::MemCpyOpt {
            src: src as usize,
            dest: dest as usize,
            len,
        });

        // Simulating the promotion to 64-bit words as required by HVM2 backend
        let words = len / 8;
        for i in 0..words {
            // Simulating tracking of each 64-bit chunk
            let _src_word = unsafe { (src as *const u64).add(i) };
            let _dest_word = unsafe { (dest as *mut u64).add(i) };
        }
    }

    /// Simulates the Interprocedural Sparse Conditional Constant Propagation (IPSCCP) pass
    /// tracking over the memory layout.
    pub fn track_ipsccp(&mut self, ptr: *mut u64, value: u64) {
        self.ops.push(MemoryOp::IpSccp {
            ptr: ptr as usize,
            value,
        });
    }
}

impl Default for IntrinsicTracker {
    fn default() -> Self {
        Self::new()
    }
}
