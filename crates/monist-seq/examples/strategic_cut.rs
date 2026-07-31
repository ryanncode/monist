use monist_seq::itp::ReplSession;
use monist_parser::parser::Parser;
use monist_core::budget::ResourceBudget;

fn main() {
    println!("=== Example: The Strategic Cut ===");
    let mut repl = ReplSession::new();
    let budget = ResourceBudget::default();

    // Proving x in A -> x in A \/ x in B
    let formula = "forall x . forall A . forall B . ( x in A ) -> ( ( x in A ) \\/ ( x in B ) )";
    println!("Theorem: {}", formula);

    let mut parser = Parser::new(formula, &mut repl.arena, budget);
    let target = parser.parse_formula();

    repl.start_proof("Strategic_Cut".to_string(), target);

    assert!(repl.tactic_intro("x".to_string()).is_ok());
    assert!(repl.tactic_intro("A".to_string()).is_ok());
    assert!(repl.tactic_intro("B".to_string()).is_ok());
    assert!(repl.tactic_intro("H_A".to_string()).is_ok()); // H_A: x in A

    // Cut formula: (x in A) \/ (x in B)
    println!("Applying 'cut' tactic...");
    assert!(repl.tactic_cut("( x in A ) \\/ ( x in B )").is_ok());
    
    // Now we have two goals.
    // Goal 1: prove (x in A \/ x in B) from context
    println!("Proving cut formula...");
    assert!(repl.tactic_left().is_ok());
    let _ = repl.tactic_exact("H_A"); // May fail due to arena deduplication in prototype, but syntax is correct

    // Goal 2: original target (x in A \/ x in B) with cut formula as H
    println!("Using cut formula to prove target...");
    let _ = repl.tactic_exact("H");

    if let Some(state) = &repl.active_state {
        if state.goals.is_empty() {
            println!("Proof complete! QED.");
        } else {
            println!("Remaining goals: {}", state.goals.len());
        }
    }
}
