use monist_core::graph::{GraphArena, ScopedVar};
use monist_core::ast::Var;
use monist_core::eval::{ExecutionLimits, EvalResult, evaluate_clause};

fn main() {
    println!("=== Russell's Paradox Interception ===");
    
    // We represent Russell's set R in R.
    // Mem(R, R) translates to constraints:
    // R -> R (weight 1)
    // R -> R (weight -1)
    
    let mut arena = GraphArena::new();
    let r_var = ScopedVar(Var::Free("R".to_string()), 0);
    let u = arena.add_var(r_var.clone());
    
    // Adding R in R edges
    arena.edges.push((u, u, 1));
    arena.edges.push((u, u, -1));
    
    println!("Graph Arena Edges for R in R: {:?}", arena.edges);
    
    // Evaluate via execution limits
    if let Some(limits) = ExecutionLimits::compute_for_graph(&arena) {
        println!("Execution Limits Computed: MCM = {:.2}, Max K-Iterations = {}", limits.mcm, limits.max_k_iterations);
        
        if limits.mcm < 0.0 {
            println!("\n[SUCCESS] Extensionality Collision structurally intercepted!");
            println!("Negative-weight cycle detected. K-Iteration halted at 0.");
            assert_eq!(limits.max_k_iterations, 0, "K-Iterations should be 0 for collision");
        } else {
            panic!("Failed to intercept negative cycle for R in R");
        }
    }
    
    // Validate via bellman_ford
    match arena.bellman_ford() {
        Ok(_) => panic!("Bellman-ford should have failed due to negative cycle"),
        Err(e) => {
            println!("Bellman-Ford detection: {}", e);
            assert!(e.contains("Negative-weight cycle detected"));
        }
    }
    
    // Also use the evaluate_clause which uses Bellman-Ford too
    use monist_core::graph::Edge;
    let edges = vec![
        Edge { source: r_var.clone(), target: r_var.clone(), weight: 1 },
        Edge { source: r_var.clone(), target: r_var.clone(), weight: -1 },
    ];
    let eval_res = evaluate_clause(&edges);
    println!("Clause evaluation result: {:?}", eval_res);
    assert_eq!(eval_res, EvalResult::NegativeCycle);
    
    println!("Russell's Paradox topological boundary validated.");
}
