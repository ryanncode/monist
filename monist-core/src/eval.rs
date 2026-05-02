use crate::graph::{Edge, ScopedVar};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvalResult {
    Success(Vec<(ScopedVar, i32)>),
    NegativeCycle,
}

pub fn evaluate_clause(edges: &[Edge]) -> EvalResult {
    // Extract unique vertices and assign them an index
    let mut vertices = Vec::new();
    for edge in edges {
        if !vertices.contains(&edge.source) {
            vertices.push(edge.source.clone());
        }
        if !vertices.contains(&edge.target) {
            vertices.push(edge.target.clone());
        }
    }
    
    let v_count = vertices.len();
    if v_count == 0 {
        return EvalResult::Success(Vec::new());
    }
    
    // Create a flat centralized array for distances
    let mut dist = vec![0; v_count];
    
    // Map ScopedVar to index
    let get_idx = |v: &ScopedVar| vertices.iter().position(|x| x == v).unwrap();

    // Convert edges to use indices for fast array relaxation
    let indexed_edges: Vec<(usize, usize, i32)> = edges.iter().map(|e| {
        (get_idx(&e.source), get_idx(&e.target), e.weight)
    }).collect();
    
    // Relax edges `v_count - 1` times (Topologically-guided K-Iteration Bound)
    for _ in 0..(v_count - 1) {
        let mut updated = false;
        for &(u, v, weight) in &indexed_edges {
            if dist[u] + weight < dist[v] {
                dist[v] = dist[u] + weight;
                updated = true;
            }
        }
        if !updated {
            break;
        }
    }
    
    // Check for negative weight cycles (Extensionality Collisions)
    for &(u, v, weight) in &indexed_edges {
        if dist[u] + weight < dist[v] {
            return EvalResult::NegativeCycle;
        }
    }
    
    let final_dist = vertices.into_iter().zip(dist.into_iter()).collect();
    EvalResult::Success(final_dist)
}
