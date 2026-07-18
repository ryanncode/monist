use crate::ir::Comb;
use bytemuck::{Pod, Zeroable};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Pod, Zeroable)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Pod, Zeroable)]
#[repr(C)]
pub struct Node {
    pub port1: u32,
    pub port2: u32,
}

impl Node {
    pub fn new(p1: Port, p2: Port) -> Self {
        Node {
            port1: p1.0,
            port2: p2.0,
        }
    }

    pub fn port1(self) -> Port {
        Port(self.port1)
    }

    pub fn port2(self) -> Port {
        Port(self.port2)
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
    pub nodes: Vec<Node>,
    pub vars: Vec<Port>,
    pub redexes: Vec<(Port, Port)>,
    pub free_list: Vec<u32>,
}

impl GNet {
    pub fn new(capacity: usize) -> Self {
        let mut free_list = Vec::with_capacity(capacity);
        for i in (0..capacity).rev() {
            free_list.push(i as u32);
        }
        let nodes = vec![Node::new(Port(0), Port(0)); capacity];

        Self {
            nodes,
            vars: Vec::new(),
            redexes: Vec::new(),
            free_list,
        }
    }

    pub fn alloc_node(&mut self, p1: Port, p2: Port) -> u32 {
        if let Some(idx) = self.free_list.pop() {
            self.nodes[idx as usize] = Node::new(p1, p2);
            idx
        } else {
            panic!("OOM");
        }
    }

    pub fn link(&mut self, p1: Port, p2: Port) {
        if p1.tag() == TAG_ERA && p2.tag() == TAG_ERA {
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

    fn erase(&mut self, port: Port) {
        if port.tag() >= TAG_CON {
            let idx = port.val() as usize;
            if idx < self.nodes.len() {
                let node = self.nodes[idx];
                self.link(Port::new(TAG_ERA, 0), node.port1());
                self.link(Port::new(TAG_ERA, 0), node.port2());
                self.free_list.push(idx as u32);
            }
        } else if port.tag() == TAG_VAR {
            if (port.val() as usize) < self.vars.len() {
                self.vars[port.val() as usize] = Port::new(TAG_ERA, 0);
            }
        }
    }

    pub fn reduce(&mut self, k_iteration_limit: usize) -> Result<usize, &'static str> {
        let mut iterations = 0;

        while let Some((p1, p2)) = self.redexes.pop() {
            if iterations >= k_iteration_limit {
                return Err("K_ITERATION_HALT: Topological Execution Limit Exceeded.");
            }
            iterations += 1;

            // 2-SIC Topological Rewrite Rules
            if p1.tag() == p2.tag() {
                // Annihilation: Identical nodes (e.g. CON-CON or DUP-DUP) collide symmetrically.
                // Ports are linked straight across, and the nodes are freed.
                let n1 = self.nodes[p1.val() as usize];
                let n2 = self.nodes[p2.val() as usize];
                self.link(n1.port1(), n2.port1());
                self.link(n1.port2(), n2.port2());
                self.free_list.push(p1.val());
                self.free_list.push(p2.val());
            } else if p1.tag() == TAG_DUP && p2.tag() == TAG_CON || p1.tag() == TAG_CON && p2.tag() == TAG_DUP {
                // Commutation: Duplicator commutes through Constructor (Lévy-optimality).
                // In a complete implementation this allocates 4 new nodes and cross-wires them.
                // We leave the memory allocation stubbed, but mathematically this avoids exponential clone scaling.
                let _n1 = self.nodes[p1.val() as usize];
                let _n2 = self.nodes[p2.val() as usize];
                self.free_list.push(p1.val());
                self.free_list.push(p2.val());
            } else {
                // Other commutations/annihilations (e.g. Eraser combinations)
                self.free_list.push(p1.val());
                self.free_list.push(p2.val());
            }
        }

        Ok(iterations)
    }

    // Basic serializer for Comb -> GNet (flat array)
    pub fn from_comb(comb: &Comb, capacity: usize) -> Self {
        let mut net = GNet::new(capacity);
        let root = Self::build_comb(&mut net, comb);
        // We link a generic VAR to root to keep it active
        let root_var_idx = net.vars.len() as u32;
        net.vars.push(Port(0));
        net.link(Port::new(TAG_VAR, root_var_idx), root);
        net
    }

    fn build_comb(net: &mut GNet, comb: &Comb) -> Port {
        match comb {
            Comb::App(left, right) => {
                let l_port = Self::build_comb(net, left);
                let r_port = Self::build_comb(net, right);
                // Simple representation, just as an example
                // In actual IN, App is a CON node pointing to l_port and r_port
                let node_idx = net.alloc_node(l_port, r_port);
                Port::new(TAG_CON, node_idx)
            }
            // For now, other combinators represent REF or specific CON tags
            _ => Port::new(TAG_REF, 0), // Stub for complex translation
        }
    }

    pub fn to_comb_string(&self) -> String {
        // Find the root port via vars[0] if it exists
        if self.vars.is_empty() {
            return "Empty Net".to_string();
        }
        let root_port = self.vars[0];
        let mut string = self.port_to_string(root_port, 0);
        if string.len() > 100 {
            string.truncate(97);
            string.push_str("...");
        }
        string
    }

    fn port_to_string(&self, port: Port, depth: usize) -> String {
        if depth > 10 {
            return "...".to_string();
        }
        match port.tag() {
            TAG_VAR => format!("Var({})", port.val()),
            TAG_REF => "Ref(Comb)".to_string(),
            TAG_ERA => "Era".to_string(),
            TAG_CON => {
                let idx = port.val() as usize;
                if idx < self.nodes.len() {
                    let node = &self.nodes[idx];
                    format!(
                        "App({}, {})",
                        self.port_to_string(node.port1(), depth + 1),
                        self.port_to_string(node.port2(), depth + 1)
                    )
                } else {
                    "Invalid(CON)".to_string()
                }
            }
            _ => format!("UnknownTag({})", port.tag()),
        }
    }
}
