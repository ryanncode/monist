use crate::graph::{Edge, GraphArena, ScopedVar};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvalResult {
    Success(Vec<(ScopedVar, i32)>),
    NegativeCycle,
}

pub fn evaluate_clause(edges: &[Edge]) -> EvalResult {
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

    let mut dist = vec![0; v_count];
    let get_idx = |v: &ScopedVar| vertices.iter().position(|x| x == v).unwrap();

    let indexed_edges: Vec<(usize, usize, i32)> = edges
        .iter()
        .map(|e| (get_idx(&e.source), get_idx(&e.target), e.weight))
        .collect();

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

    for &(u, v, weight) in &indexed_edges {
        if dist[u] + weight < dist[v] {
            return EvalResult::NegativeCycle;
        }
    }

    let final_dist = vertices.into_iter().zip(dist.into_iter()).collect();
    EvalResult::Success(final_dist)
}

#[derive(Debug, Clone)]
pub struct ExecutionLimits {
    pub max_k_iterations: usize,
    pub mcm: f64,
}

impl ExecutionLimits {
    pub fn compute_for_graph(graph: &GraphArena) -> Option<Self> {
        let n = graph.vars.len();
        if n == 0 {
            return None;
        }

        // Karp's Minimum Cycle Mean (MCM) Algorithm
        // DP array: dp[k][v] = min weight of path of length k to v
        let mut dp = vec![vec![i32::MAX / 2; n]; n + 1];
        for v in 0..n {
            dp[0][v] = 0;
        }

        for k in 1..=n {
            for &(u, v, w, _) in &graph.edges {
                if dp[k - 1][u] + w < dp[k][v] {
                    dp[k][v] = dp[k - 1][u] + w;
                }
            }
        }

        let mut mcm: f64 = f64::INFINITY;
        let mut has_cycle = false;

        for v in 0..n {
            if dp[n][v] >= i32::MAX / 4 {
                continue;
            }
            let mut min_val: f64 = f64::NEG_INFINITY;
            for k in 0..n {
                if dp[k][v] >= i32::MAX / 4 {
                    continue;
                }
                let val = (dp[n][v] - dp[k][v]) as f64 / (n - k) as f64;
                if val > min_val {
                    min_val = val;
                }
            }
            if min_val < mcm {
                mcm = min_val;
                has_cycle = true;
            }
        }

        if !has_cycle {
            mcm = 0.0;
        }

        // K-Iteration based on Pigeonhole Principle
        let max_iterations = if mcm < 0.0 {
            // Negative cycle indicates Extensionality Collision, halt early.
            0
        } else {
            // Safe geometric limits
            n * 2
        };

        Some(ExecutionLimits {
            max_k_iterations: max_iterations,
            mcm,
        })
    }
}
