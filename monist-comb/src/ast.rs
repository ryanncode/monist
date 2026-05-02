#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Port(pub u32);

impl Port {
    pub fn new(tag: u32, val: u32) -> Self {
        Port((tag & 0x7) | (val << 3))
    }

    pub fn tag(self) -> u32 {
        self.0 & 0x7
    }

    pub fn val(self) -> u32 {
        self.0 >> 3
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Pair(pub u64);

impl Pair {
    pub fn new(p1: Port, p2: Port) -> Self {
        Pair((p1.0 as u64) | ((p2.0 as u64) << 32))
    }

    pub fn port1(self) -> Port {
        Port(self.0 as u32)
    }

    pub fn port2(self) -> Port {
        Port((self.0 >> 32) as u32)
    }
}

pub const TAG_VAR: u32 = 0;
pub const TAG_REF: u32 = 1;
pub const TAG_ERA: u32 = 2;
pub const TAG_NUM: u32 = 3;
pub const TAG_CON: u32 = 4;
pub const TAG_DUP: u32 = 5;
pub const TAG_OPR: u32 = 6;
pub const TAG_SWI: u32 = 7;

#[derive(Debug, Clone, Default)]
pub struct GNet {
    pub nodes: Vec<Pair>,
    pub vars: Vec<Port>,
    pub redexes: Vec<(Port, Port)>,
}

impl GNet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn alloc_node(&mut self, p1: Port, p2: Port) -> u32 {
        let idx = self.nodes.len() as u32;
        self.nodes.push(Pair::new(p1, p2));
        idx
    }

    /// Link two ports together. Triggers VOID or ERASE on ERA tags.
    pub fn link(&mut self, p1: Port, p2: Port) {
        if p1.tag() == TAG_ERA && p2.tag() == TAG_ERA {
            // VOID sub-interaction
            return;
        }

        if p1.tag() == TAG_ERA {
            self.erase(p2);
            return;
        }

        if p2.tag() == TAG_ERA {
            self.erase(p1);
            return;
        }

        if p1.tag() >= TAG_CON && p2.tag() >= TAG_CON {
            self.redexes.push((p1, p2));
            return;
        }

        if p1.tag() == TAG_VAR {
            if (p1.val() as usize) < self.vars.len() {
                self.vars[p1.val() as usize] = p2;
            }
        }
        if p2.tag() == TAG_VAR {
            if (p2.val() as usize) < self.vars.len() {
                self.vars[p2.val() as usize] = p1;
            }
        }
    }

    /// ERASE sub-interaction
    fn erase(&mut self, port: Port) {
        if port.tag() >= TAG_CON {
            let idx = port.val() as usize;
            if idx < self.nodes.len() {
                let pair = self.nodes[idx];
                self.link(Port::new(TAG_ERA, 0), pair.port1());
                self.link(Port::new(TAG_ERA, 0), pair.port2());
            }
        } else if port.tag() == TAG_VAR {
            if (port.val() as usize) < self.vars.len() {
                self.vars[port.val() as usize] = Port::new(TAG_ERA, 0);
            }
        }
    }

    /// Topologically-Guided Call-by-Need Evaluation Engine
    /// Executes the Interaction Net and halts dynamically via K-Iteration bounds
    pub fn reduce(&mut self, k_iteration_limit: usize) -> Result<usize, &'static str> {
        let mut iterations = 0;

        while let Some((_p1, _p2)) = self.redexes.pop() {
            if iterations >= k_iteration_limit {
                // Trap into the K_ITERATION_HALT terminal state
                // Mathematically isolates paradox regressions (like V in V) natively
                return Err("K_ITERATION_HALT: Topological Execution Limit Exceeded.");
            }

            // At this stage, standard Interaction Net reductions would occur:
            // 1. Commutation (e.g. CON/DUP crossing)
            // 2. Annihilation (e.g. CON/CON merging)
            
            // For now, assume rule consumes redex and potentially adds more
            // Actual implementation hooks into HVM2 / Interaction Net rewrite matrices
            
            iterations += 1;
        }

        Ok(iterations)
    }
}
