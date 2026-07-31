use monist_seq::itp::ReplSession;
use monist_parser::parser::Parser;
use monist_core::budget::ResourceBudget;

fn main() {
    println!("=== Example: Monotonicity of the Powerset (Tactics Demo) ===");
    let mut repl = ReplSession::new();
    let budget = ResourceBudget::default();

    // To simulate Powersets without macro compilation, we prove a structural logic representation:
    // (A -> B) & (PA -> A) & (PB -> B) -> (PA -> PB)
    // We will use stratify and defer to manipulate the goals.
    let formula = "forall A . forall B . forall PA . forall PB . ( ( A -> B ) & ( PA -> A ) & ( PB -> B ) ) -> ( PA -> PB )";
    println!("Theorem: {}", formula);

    let mut parser = Parser::new(formula, &mut repl.arena, budget);
    let target = parser.parse_formula();

    repl.start_proof("Pow_Mono".to_string(), target);

    assert!(repl.tactic_intro("A".to_string()).is_ok());
    assert!(repl.tactic_intro("B".to_string()).is_ok());
    assert!(repl.tactic_intro("PA".to_string()).is_ok());
    assert!(repl.tactic_intro("PB".to_string()).is_ok());
    
    // Destruct the conjunction into three hypotheses
    assert!(repl.tactic_intro("H".to_string()).is_ok());
    
    println!("Using defer to rotate the active goal state...");
    assert!(repl.tactic_defer().is_ok());

    println!("Checking geometric validity of the proof state with stratify...");
    // `stratify` checks if the topological bounds hold without further deduction.
    // In this abstract formulation, it evaluates the DAG context natively.
    if repl.tactic_stratify().is_ok() {
        println!("Stratification successful. Topological bounds verified.");
    } else {
        println!("Stratification required manual proof steps.");
    }

    if let Some(state) = &repl.active_state {
        if state.goals.is_empty() {
            println!("Proof complete! QED.");
        } else {
            println!("Remaining goals: {}", state.goals.len());
        }
    }
}
