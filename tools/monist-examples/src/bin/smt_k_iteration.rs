use monist_core::ast::FormulaArena;
use monist_core::graph::{GraphArena, extract_constraints_aux};
use monist_core::smt::export_smt_lib;
use monist_parser::parser::Parser;

fn main() {
    println!("=== K-Iteration Safety Bound via SMT-LIB ===");
    println!("Demonstrating stratification verification for V \\in V.\n");

    let formula_str = "V in V";
    println!("Input Formula: {}", formula_str);

    let mut arena = FormulaArena::new();
    let mut parser = Parser::new(formula_str, &mut arena);
    let root_idx = parser.parse_formula();

    let constraints = extract_constraints_aux(&arena, root_idx, 0);
    let mut graph = GraphArena::from_constraints(&constraints);
    graph.collapse_scc_0_weight();

    // Check with Bellman-Ford
    match graph.bellman_ford() {
        Ok(_) => println!("Result: Stratification successful (Unexpected for V \\in V!)."),
        Err(e) => println!("Result: Evaluation intercepted. {}", e),
    }

    println!("\nGenerating SMT-LIB Mathematical Trace:");
    println!("--------------------------------------");
    let smt_output = export_smt_lib(&graph, "K_Iteration_V_in_V");
    println!("{}", smt_output);
    println!("--------------------------------------");
    println!("This generated SMT-LIB code can be passed to Lean 4, Z3, or CVC5 to independently verify the termination bound.");
}
