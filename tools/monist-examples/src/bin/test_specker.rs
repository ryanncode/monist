use monist_core::ast::FormulaArena;
use monist_core::graph::{extract_constraints_aux, GraphArena};
use monist_core::smt::export_smt_lib;
use monist_parser::parser::Parser;

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
    let mut parser = Parser::new(formula, &mut arena, monist_core::budget::ResourceBudget::default());
    let root_idx = parser.parse_formula();

    let constraints = extract_constraints_aux(&arena, root_idx, 0, false, &monist_core::budget::ResourceBudget::default(), &mut 0);
    let mut graph = GraphArena::from_constraints(&constraints);

    println!(">> Executing Kosaraju SCC Flattening...");
    graph.collapse_scc_0_weight();

    println!(">> Running Bellman-Ford Shortest-Path Topology Check...");
    let bf_result = graph.evaluate_topology();

    println!("=== Stratification Witness (SMT-LIB format) ===");
    match &bf_result {
        Ok((success_depths, sc_actions, _, _)) => {
            println!(
                "{}",
                export_smt_lib(&graph, "Specker_Collision", None, sc_actions, Some(success_depths))
            );
        }
        Err(collision_trace) => {
            println!(
                "{}",
                export_smt_lib(&graph, "Specker_Collision", Some(collision_trace), &[], None)
            );
        }
    }
    println!("===============================================\n");

    match bf_result {
        Ok((_, _, _, _)) => println!("[FAIL] The unstratified choice function was evaluated successfully (it shouldn't)."),
        Err(e) => {
            println!("[SUCCESS] Native topological limits intercepted the paradox!");
            println!("Error: {}", e);
        }
    }
}
