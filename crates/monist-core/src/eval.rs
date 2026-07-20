use crate::graph::{Edge, GraphArena, ScopedVar};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvalResult {
    Success(Vec<(ScopedVar, i32)>),
    NegativeCycle,
    NumericOverflow,
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

    let mut dist: Vec<i64> = vec![0; v_count];
    let get_idx = |v: &ScopedVar| vertices.iter().position(|x| x == v).unwrap();

    let indexed_edges: Vec<(usize, usize, i64)> = edges
        .iter()
        .map(|e| (get_idx(&e.source), get_idx(&e.target), e.weight as i64))
        .collect();

    for _ in 0..(v_count - 1) {
        let mut updated = false;
        for &(u, v, weight) in &indexed_edges {
            if let Some(new_dist) = dist[u].checked_add(weight) {
                if new_dist < dist[v] {
                    dist[v] = new_dist;
                    updated = true;
                }
            } else {
                return EvalResult::NumericOverflow;
            }
        }
        if !updated {
            break;
        }
    }

    for &(u, v, weight) in &indexed_edges {
        if let Some(new_dist) = dist[u].checked_add(weight) {
            if new_dist < dist[v] {
                return EvalResult::NegativeCycle;
            }
        } else {
            return EvalResult::NumericOverflow;
        }
    }

    let final_dist = vertices.into_iter().zip(dist.into_iter().map(|d| d as i32)).collect();
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

        const INF: i64 = i64::MAX / 2;

        // Karp's Minimum Cycle Mean (MCM) Algorithm
        // DP array: dp[k][v] = min weight of path of length k to v
        let mut dp = vec![vec![INF; n]; n + 1];
        for v in 0..n {
            dp[0][v] = 0;
        }

        for k in 1..=n {
            for &(u, v, w, _) in &graph.edges {
                if dp[k - 1][u] == INF {
                    continue;
                }
                if let Some(new_dist) = dp[k - 1][u].checked_add(w as i64) {
                    if new_dist < dp[k][v] {
                        dp[k][v] = new_dist;
                    }
                } else {
                    return None; // numeric overflow
                }
            }
        }

        let mut mcm: f64 = f64::INFINITY;
        let mut has_cycle = false;

        for v in 0..n {
            if dp[n][v] == INF {
                continue;
            }
            let mut min_val: f64 = f64::NEG_INFINITY;
            for k in 0..n {
                if dp[k][v] == INF {
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
