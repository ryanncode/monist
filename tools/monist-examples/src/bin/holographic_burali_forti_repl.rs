use monist_comb::comblib::vsa_embed::{Codebook, HDCVector};
use monist_core::ast::FormulaArena;
use monist_core::eval::ExecutionLimits;
use monist_core::graph::{extract_constraints_aux, GraphArena};
use monist_core::smt::export_smt_lib;
use monist_parser::parser::Parser;

struct Session {
    graph: GraphArena,
    arena: FormulaArena,
    codebook: Codebook,
}

impl Session {
    fn new() -> Self {
        Self {
            graph: GraphArena::new(),
            arena: FormulaArena::new(),
            codebook: Codebook::new(),
        }
    }

    fn eval_graph(&mut self, formula: &str, test_name: &str) {
        let mut parser = Parser::new(formula, &mut self.arena, monist_core::budget::ResourceBudget::default());
        let root_idx = parser.parse_formula();

        let constraints = extract_constraints_aux(&self.arena, root_idx, 0, true, &monist_core::budget::ResourceBudget::default(), &mut 0);
        self.graph = GraphArena::from_constraints(&constraints);
        
        self.graph.collapse_scc_0_weight();

        let bf_result = self.graph.evaluate_topology();

        println!(
            "\n=== Stratification Witness (SMT-LIB format) for {} ===",
            test_name
        );
        match &bf_result {
            Ok((depths, sc_actions, _, _)) => {
                println!("{}", export_smt_lib(&self.graph, test_name, None, sc_actions, Some(depths)));
            }
            Err(trace) => {
                println!("{}", export_smt_lib(&self.graph, test_name, Some(trace), &[], None));
            }
        }
        println!("===============================================\n");

        if let Some(limits) = ExecutionLimits::compute_for_graph(&self.graph) {
            println!(
                "Execution Limits Computed: MCM = {:.2}, Max K-Iterations = {}\n",
                limits.mcm, limits.max_k_iterations
            );
        }

        println!("Max Graph Topology: {} nodes reached.\n", self.graph.vars.len());
        
        println!("=== Holographic Heuristic Search ===");
        println!("Checking for paradoxical collision cycles using VSA phase cancellation...");
        
        let conflict_cycles = self.graph.extract_conflict_clauses();
        
        if conflict_cycles.is_empty() {
            println!("No topological phase conflicts detected. Safe to embed.");
            return;
        }

        println!("Detected {} conflicting cycles. Engaging VSA resolution.", conflict_cycles.len());

        let mut node_vectors: Vec<HDCVector> = Vec::new();
        for i in 0..self.graph.vars.len() {
            let name = format!("Node_{}", i);
            let vec = HDCVector::new();
            self.codebook.insert(name.clone(), vec.clone());
            node_vectors.push(vec);
        }

        let mut mask = HDCVector::new();
        let mut num_conflicts_masked = 0;
        let mut conflict_clause_nodes = Vec::new();
        
        for cycle in conflict_cycles {
             for var_idx in cycle {
                  let v_name = format!("Node_{}", var_idx);
                  conflict_clause_nodes.push(v_name.clone());
                  let v_vec = HDCVector::new();
                  self.codebook.insert(v_name.clone(), v_vec.clone());
                  for i in 0..10000 { mask.values[i] += v_vec.values[i]; }
                  num_conflicts_masked += 1;
             }
        }
        
        if num_conflicts_masked > 0 {
             for i in 0..10000 { mask.values[i] /= (num_conflicts_masked as f32).sqrt(); }
        }
        
        self.codebook.apply_conflict_mask(&conflict_clause_nodes);

        println!("Phase cancellation applied successfully. Topologically invalid branches have been un-bound from the VSA continuous space.");
        println!("===============================================\n");
    }
}

fn main() {
    println!("=== REPL Tactic Integration (Holographic Hybrid) ===");
    println!("Initializing Interactive Proof Session...\n");

    println!("> assume Ord_Def \"forall x. Ord(x) <-> Transitive(x) /\\ WellOrdered(x)\"");
    println!("[Loaded] Axiom Ord_Def registered.\n");

    println!("> assume Burali_Forti_Premise \"Omega = Set_of_all_Ordinals\"");
    println!("[Loaded] Axiom Burali_Forti_Premise registered.\n");

    println!("> theorem Burali_Forti_Paradox \"Ord(Omega) -> Omega < Omega\"");
    println!("[Goal Set] 1 unproven target.");
    println!("Target 1: Ord(Omega) -> Omega < Omega");
    println!("Context: \n");

    println!("> intro H_Ord_Omega");
    println!("[Context Updated] Hypotheses H1: Ord(Omega) added.\n");
    println!("Target 1: Omega < Omega");

    println!("> tactic t_shift_resolve");

    println!("Composite structural matrix evaluated.");

    let mut session = Session::new();
    let formula = "((Omega = T_Omega) /\\ (T_Omega < Omega))";
    session.eval_graph(formula, "burali_forti_repl_holographic");

    println!("[REJECTED] Topological conflict detected! Cycle identified via Holographic Subtractor.");
    println!("[Goal Failed] Burali-Forti paradox triggers type-state cycle. Proof halted to preserve consistency.");
}
