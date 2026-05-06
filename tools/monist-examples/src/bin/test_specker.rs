use monist_core::ast::FormulaArena;
use monist_parser::parser::Parser;
use monist_core::graph::{GraphArena, extract_constraints_aux};
use monist_core::smt::export_smt_lib;

fn main() {
    println!("============================================================");
    println!("   Test Specker's Theorem (Empirical Parity Collision)      ");
    println!("============================================================\n");

    let formula = "{ F | (((m in F /\\ p1 in F) /\\ p2 in F) /\\ (z in p1 -> (w in z -> w in m))) /\\ (z2 in p2 -> (w2 in z2 -> w2 in p1)) }";
    
    println!(">> Parsing Specker Formula:");
    println!("   {}\n", formula);
    println!("This formula attempts to define a Universal Choice function F over");
    println!("distinct cardinal boundaries (m, p1 = 2^m, and p2 = 2^{{2^m}}).");
    println!("Because Pure NF strictly regulates topological parity shifts,");
    println!("evaluating this should autonomously trigger a Negative-Weight Cycle.\n");

    let mut arena = FormulaArena::new();
    let mut parser = Parser::new(formula, &mut arena);
    let root_idx = parser.parse_formula();

    let constraints = extract_constraints_aux(&arena, root_idx, 0);
    let mut graph = GraphArena::from_constraints(&constraints);
    
    println!(">> Executing Kosaraju SCC Flattening...");
    graph.collapse_scc_0_weight();
    
    println!("=== Stratification Witness (SMT-LIB format) ===");
    println!("{}", export_smt_lib(&graph, "Specker_Collision"));
    println!("===============================================\n");

    println!(">> Running Bellman-Ford Shortest-Path Topology Check...");
    match graph.bellman_ford() {
        Ok(_) => println!("[FAIL] The unstratified choice function was incorrectly allowed."),
        Err(e) => {
            println!("[SUCCESS] Native topological limits intercepted the paradox!");
            println!("Error: {}", e);
        }
    }
}
