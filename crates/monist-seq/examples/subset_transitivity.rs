use monist_seq::itp::ReplSession;
use monist_parser::parser::Parser;
use monist_core::budget::ResourceBudget;

fn main() {
    println!("=== Example: Subset Transitivity ===");
    let mut repl = ReplSession::new();
    let budget = ResourceBudget::default();

    let formula = "forall A . forall B . forall C . forall z . ( ( z in A -> z in B ) /\\ ( z in B -> z in C ) ) -> ( z in A -> z in C )";
    println!("Theorem: {}", formula);

    let mut parser = Parser::new(formula, &mut repl.arena, budget);
    let target = parser.parse_formula();

    repl.start_proof("Subset_Trans".to_string(), target);

    assert!(repl.tactic_intro("A".to_string()).is_ok());
    assert!(repl.tactic_intro("B".to_string()).is_ok());
    assert!(repl.tactic_intro("C".to_string()).is_ok());
    assert!(repl.tactic_intro("z".to_string()).is_ok());
    assert!(repl.tactic_intro("H".to_string()).is_ok());

    assert!(repl.tactic_destruct("H", "H1".to_string(), "H2".to_string()).is_ok());
    
    // ( z in A -> z in C )
    assert!(repl.tactic_intro("Hz".to_string()).is_ok());

    // Target is now (z in C). H2 is (z in B -> z in C).
    assert!(repl.tactic_apply("H2").is_ok());

    // Target is now (z in B). H1 is (z in A -> z in B).
    assert!(repl.tactic_apply("H1").is_ok());

    // Target is now (z in A). Hz is (z in A).
    assert!(repl.tactic_exact("Hz").is_ok());

    if let Some(state) = &repl.active_state {
        if state.goals.is_empty() {
            println!("Proof complete! QED.");
        } else {
            println!("Remaining goals: {}", state.goals.len());
        }
    }
}
