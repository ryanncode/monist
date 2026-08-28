use monist_core::budget::ResourceBudget;
use monist_parser::parser::Parser;
use monist_seq::itp::ReplSession;

fn main() {
    println!("=== ITP Proof: Stratified Universal Set Comprehension V ===");
    let mut repl = ReplSession::new();
    let budget = ResourceBudget::default();

    // Universal Set: forall x. (x = x)
    let formula = "forall x . ( x = x )";
    println!("Target Theorem: {}", formula);

    let mut parser = Parser::new(formula, &mut repl.arena, budget);
    let target = parser.parse_formula();

    repl.start_proof("Universal_Set_Refl".to_string(), target);

    println!("\nExecuting tactic_intro(\"a\")...");
    assert!(repl.tactic_intro("a".to_string()).is_ok());

    println!("Executing tactic_refl() to close equality via 0-weight Tarjan SCC...");
    assert!(repl.tactic_refl().is_ok());

    if let Some(state) = &repl.active_state {
        if state.goals.is_empty() {
            println!("\n[SUCCESS] Proof closed cleanly! QED.");
        } else {
            panic!("Expected 0 remaining goals, found {}", state.goals.len());
        }
    }
}

