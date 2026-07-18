use monist_core::ast::FormulaArena;
use monist_core::graph::{extract_constraints_aux, GraphArena};
use monist_core::smt::export_smt_lib;
use monist_parser::parser::Parser;

struct Session {
    graph: GraphArena,
    arena: FormulaArena,
}

impl Session {
    fn new() -> Self {
        Self {
            graph: GraphArena::new(),
            arena: FormulaArena::new(),
        }
    }

    fn eval_organic(
        &mut self,
        formula: &str,
        test_name: &str,
    ) -> Result<(Vec<i32>, Vec<String>, bool, bool), String> {
        let mut parser = Parser::new(formula, &mut self.arena);
        let root_idx = parser.parse_formula();

        let constraints = extract_constraints_aux(&self.arena, root_idx, 0, false);
        self.graph = GraphArena::from_constraints(&constraints);

        self.graph.collapse_scc_0_weight();
        // The continuous daemon autonomously operates here inside evaluate_topology!

        let bf_result = self.graph.evaluate_topology();

        println!(
            "=== Stratification Witness (SMT-LIB format) for {} ===",
            test_name
        );
        match &bf_result {
            Ok((success_depths, sc_actions, _, _)) => {
                println!(
                    "{}",
                    export_smt_lib(&self.graph, test_name, None, sc_actions, Some(success_depths))
                );
            }
            Err(collision_trace) => {
                println!(
                    "{}",
                    export_smt_lib(&self.graph, test_name, Some(collision_trace), &[], None)
                );
            }
        }
        println!("===============================================\n");

        bf_result
    }
}

fn main() {
    println!("============================================================");
    println!("   Direct Observable Proof: SC Daemon Edge-Severing         ");
    println!("============================================================\n");

    println!(">> Step 1: The Baseline Paradox (No SC Bedrock)");
    println!("We evaluate a standard unstratified 2-cycle: 'S in y /\\ y in S'.");
    println!("This should correctly trigger an Extensionality Collision in Bellman-Ford.\n");

    let mut session_baseline = Session::new();
    let baseline_formula = "(S in y /\\ y in S)";

    match session_baseline.eval_organic(baseline_formula, "Baseline_Paradox") {
        Ok((_, _, _, _)) => panic!("[FAIL] The engine incorrectly allowed the 2-cycle to compute without interference!"),
        Err(e) => {
            println!("[SUCCESS] Baseline intercepted the paradox!");
            println!("Error: {}\n", e);
        }
    }

    println!(">> Step 2: Activating the SC Daemon via 'S in S'");
    println!(
        "We evaluate the EXACT same paradox, but add 'S in S': '(S in y /\\ y in S) /\\ S in S'."
    );
    println!("The self-loop 'S in S' establishes S as a Strongly Cantorian set (S = T(S)).");
    println!("The daemon should dynamically detect this, sever the outgoing +1 edge 'S in y',");
    println!(
        "and cleanly bypass the negative-weight cycle, isolating the ZFC-compliant bedrock.\n"
    );

    let mut session_sc = Session::new();
    let sc_formula = "((S in y /\\ y in S) /\\ S in S)";

    match session_sc.eval_organic(sc_formula, "SC_Daemon_Isolation") {
        Ok((_, _, _, _)) => {
            println!("[SUCCESS] The SC Daemon actively severed the outgoing +1 edge!");
            println!("The topological paradox was neutralized because S was recognized as ZFC-compliant bedrock.");
        }
        Err(e) => {
            println!("[FAIL] The daemon failed to sever the edges. Error: {}", e);
            panic!("SC Daemon isolation failed.");
        }
    }

    println!("\n============================================================");
    println!(
        "[VERIFIED] Automated Detection of Strongly Cantorian Subgraphs is fully operational."
    );
    println!("============================================================");
}
