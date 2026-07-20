#[derive(Debug, Clone, Copy)]
pub struct ResourceBudget {
    pub max_bytes: usize,
    pub max_tokens: usize,
    pub max_depth: usize,
    pub max_ast_nodes: usize,
    pub max_graph_edges: usize,
}

impl Default for ResourceBudget {
    fn default() -> Self {
        Self {
            max_bytes: 50_000,
            max_tokens: 10_000,
            max_depth: 500,
            max_ast_nodes: 20_000,
            max_graph_edges: 50_000,
        }
    }
}
