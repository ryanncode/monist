use indicatif::{ProgressBar, ProgressStyle};
use monist_comb::comblib::frege_russell::{num0, num1};
use monist_core::ast::{Atomic, Formula, FormulaArena, Var};
use monist_core::graph::{Constraint, GraphArena, ScopedVar};
use monist_core::smt::export_smt_lib;
use std::time::Duration;

struct Session {
    pb: ProgressBar,
}

impl Session {
    fn new() -> Self {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
                .template("{spinner:.green} [{elapsed_precise}] {msg}")
                .unwrap(),
        );
        pb.enable_steady_tick(Duration::from_millis(100));
        Self { pb }
    }

    fn eval_and_export(&mut self, graph: &mut GraphArena, test_name: &str) {
        self.pb.set_message(format!("Evaluating Frege-Russell {}", test_name));
        let (trace, sc_actions, success_depths) = match graph.bellman_ford() {
            Ok((depths, actions)) => {
                self.pb.finish_with_message(format!(
                    "{} topology resolved. Graph is coherent. SC bedrock bounds enforced.",
                    test_name
                ));
                (None, actions, Some(depths))
            }
            Err(e) => {
                self.pb
                    .finish_with_message(format!("{} Paradox detected: {}", test_name, e));
                (Some(e), vec![], None)
            }
        };

        if let Some(ref t) = trace {
            println!("Paradox Trace: {}", t);
        }

        let smt_lib = export_smt_lib(
            graph,
            test_name,
            trace.as_deref(),
            &sc_actions,
            success_depths.as_deref(),
        );
        println!("\n=== SMT-LIB Differential Export ({}) ===\n{}", test_name, smt_lib);
    }
}

fn main() {
    println!("=== Testing Frege-Russell Numerals CombLib Encodings ===");
    
    // Test 1: num0 geometry
    let mut session = Session::new();
    let mut arena = FormulaArena::new();
    
    let _c_num0 = num0();
    
    let n0_var = Var::Free("Num0".to_string());
    
    // We want to test that num0 represents the set of all empty sets.
    // For a minimal graph to execute, we can add a constraint like Num0 e Num0
    // just to see its topological behavior under Stratification.
    let _f1 = arena.add(Formula::Atom(Atomic::Mem(n0_var.clone(), n0_var.clone())));
    
    let constraints = vec![
        Constraint { v1: ScopedVar(n0_var.clone(), 0), v2: ScopedVar(n0_var.clone(), 0), weight: -1, in_comp: true },
    ];
    let mut graph0 = GraphArena::from_constraints(&constraints);
    session.eval_and_export(&mut graph0, "Num0_Self_Loop");
    
    // Test 2: num1 geometry
    let mut session = Session::new();
    let _c_num1 = num1();
    
    let n1_var = Var::Free("Num1".to_string());
    let constraints1 = vec![
        Constraint { v1: ScopedVar(n0_var.clone(), 0), v2: ScopedVar(n1_var.clone(), 0), weight: -1, in_comp: true },
    ];
    let mut graph1 = GraphArena::from_constraints(&constraints1);
    session.eval_and_export(&mut graph1, "Num0_in_Num1");
}
