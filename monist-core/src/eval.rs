use std::collections::HashMap;
use crate::graph::{Edge, ScopedVar};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvalResult {
    Success(HashMap<ScopedVar, i32>),
    NegativeCycle,
}

pub fn evaluate_clause(edges: &[Edge]) -> EvalResult {
    let mut dist: HashMap<ScopedVar, i32> = HashMap::new();
    
    // Extract unique vertices
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
        return EvalResult::Success(dist);
    }
    
    // Initialize distances. Starting with 0 allows us to compute valid relative stratifications.
    for v in &vertices {
        dist.insert(v.clone(), 0);
    }
    
    // Relax edges `v_count - 1` times
    for _ in 0..(v_count - 1) {
        let mut updated = false;
        for edge in edges {
            let u_dist = *dist.get(&edge.source).unwrap();
            let v_dist = *dist.get(&edge.target).unwrap();
            if u_dist + edge.weight < v_dist {
                dist.insert(edge.target.clone(), u_dist + edge.weight);
                updated = true;
            }
        }
        if !updated {
            break;
        }
    }
    
    // Check for negative weight cycles
    for edge in edges {
        let u_dist = *dist.get(&edge.source).unwrap();
        let v_dist = *dist.get(&edge.target).unwrap();
        if u_dist + edge.weight < v_dist {
            return EvalResult::NegativeCycle;
        }
    }
    
    EvalResult::Success(dist)
}
