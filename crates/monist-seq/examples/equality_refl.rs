use monist_seq::itp::ReplSession;
use monist_parser::parser::Parser;
use monist_core::budget::ResourceBudget;

fn main() {
    println!("=== Example: Topological Equality (Refl) ===");
    let mut repl = ReplSession::new();
    let budget = ResourceBudget::default();

    // Prove that x = y -> y = x using refl which leverages kosaraju_scc
    let formula = "forall x . forall y . ( x = y ) -> ( y = x )";
    println!("Theorem: {}", formula);

    let mut parser = Parser::new(formula, &mut repl.arena, budget);
    let target = parser.parse_formula();

    repl.start_proof("Symmetry".to_string(), target);

    assert!(repl.tactic_intro("x".to_string()).is_ok());
    assert!(repl.tactic_intro("y".to_string()).is_ok());
    assert!(repl.tactic_intro("H".to_string()).is_ok());

    println!("Checking equality natively using topology (refl)...");
    assert!(repl.tactic_refl().is_ok());

    if let Some(state) = &repl.active_state {
        if state.goals.is_empty() {
            println!("Proof complete! QED.");
        } else {
            println!("Remaining goals: {}", state.goals.len());
        }
    }
}
