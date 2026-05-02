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
}
