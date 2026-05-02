use monist_core::ast::FormulaArena;
use monist_parser::parser::Parser;
use monist_core::graph::{GraphArena, extract_constraints_aux};

fn main() {
    let formula = "{ F | (((m in F /\\ p1 in F) /\\ p2 in F) /\\ (z in p1 -> (w in z -> w in m))) /\\ (z2 in p2 -> (w2 in z2 -> w2 in p1)) }";
    let mut arena = FormulaArena::new();
    let mut parser = Parser::new(formula, &mut arena);
    let root_idx = parser.parse_formula();

    let constraints = extract_constraints_aux(&arena, root_idx, 0);
    let mut graph = GraphArena::from_constraints(&constraints);
    graph.collapse_scc_0_weight();

    match graph.bellman_ford() {
        Ok(_) => println!("OK"),
        Err(e) => println!("Error: {}", e),
    }
}
