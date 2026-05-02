use std::collections::HashMap;

/// Represents the state of a node in the Program Semantic Graph (PSG).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NodeState {
    /// Actively part of the computation graph.
    Live,
    /// Excluded from active computation but retains annotations for potential reactivation.
    Latent,
    /// Newly created, not yet integrated into the graph's execution flow.
    Fresh,
}

/// A node within the Program Semantic Graph.
#[derive(Debug, Clone)]
pub struct PsgNode {
    /// Unique identifier for the node.
    pub id: usize,
    /// The current state of the node.
    pub state: NodeState,
    /// Arbitrary metadata or annotations attached to the node, preserved through passes.
    pub annotations: HashMap<String, String>,
}

impl PsgNode {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            state: NodeState::Fresh,
            annotations: HashMap::new(),
        }
    }
}

/// The Program Semantic Graph (PSG), preserving semantic metadata through multi-stage lowering.
#[derive(Debug, Default)]
pub struct ProgramSemanticGraph {
    nodes: HashMap<usize, PsgNode>,
    next_id: usize,
}

impl ProgramSemanticGraph {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a new node to the graph in the `Fresh` state.
    pub fn add_node(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.nodes.insert(id, PsgNode::new(id));
        id
    }

    /// Updates the state of a node.
    pub fn set_state(&mut self, id: usize, state: NodeState) {
        if let Some(node) = self.nodes.get_mut(&id) {
            node.state = state;
        }
    }

    /// Adds an annotation to a node, preserving dimensional/semantic metadata.
    pub fn add_annotation(&mut self, id: usize, key: impl Into<String>, value: impl Into<String>) {
        if let Some(node) = self.nodes.get_mut(&id) {
            node.annotations.insert(key.into(), value.into());
        }
    }

    /// Retrieves a node by ID.
    pub fn get_node(&self, id: usize) -> Option<&PsgNode> {
        self.nodes.get(&id)
    }
}
