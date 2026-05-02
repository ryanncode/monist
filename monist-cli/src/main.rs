use monist_core::ast::FormulaArena;
use monist_core::graph::{GraphArena, extract_constraints_aux};
use monist_core::smt::export_smt_lib;
use monist_parser::parser::Parser;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut export_smt = false;
    let mut formula_str = None;

    for arg in args.iter().skip(1) {
        if arg == "--export-smt" {
            export_smt = true;
        } else if formula_str.is_none() {
            formula_str = Some(arg.clone());
        }
    }

    let formula_str = match formula_str {
        Some(s) => s,
        None => {
            eprintln!("Usage: monist-cli [--export-smt] <formula>");
            std::process::exit(1);
        }
    };

    let mut arena = FormulaArena::new();
    let mut parser = Parser::new(&formula_str, &mut arena);
    let root_idx = parser.parse_formula();

    let constraints = extract_constraints_aux(&arena, root_idx, 0);
    let mut graph = GraphArena::from_constraints(&constraints);
    graph.collapse_scc_0_weight();

    if export_smt {
        let smt_output = export_smt_lib(&graph, "cli_input");
        println!("{}", smt_output);
    } else {
        match graph.bellman_ford() {
            Ok(_) => println!("Stratification successful."),
            Err(e) => println!("Error: {}", e),
        }
    }
}
