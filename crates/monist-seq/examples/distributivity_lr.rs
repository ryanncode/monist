use monist_seq::itp::ReplSession;
use monist_parser::parser::Parser;
use monist_core::budget::ResourceBudget;

fn main() {
    println!("=== Example: Distributivity (Left to Right) ===");
    let mut repl = ReplSession::new();
    let budget = ResourceBudget::default();

    let formula = "forall x . forall A . forall B . forall C . ( x in A /\\ ( x in B \\/ x in C ) ) -> ( ( x in A /\\ x in B ) \\/ ( x in A /\\ x in C ) )";
    println!("Theorem: {}", formula);

    let mut parser = Parser::new(formula, &mut repl.arena, budget);
    let target = parser.parse_formula();
    println!("Parsed target index: {}", target);
    println!("Target formula: {:?}", repl.arena.get(target));
    println!("Target formula: {:?}", repl.arena.get(target));

    repl.start_proof("Distributivity_LR".to_string(), target);

    assert!(repl.tactic_intro("x".to_string()).is_ok());
    assert!(repl.tactic_intro("A".to_string()).is_ok());
    assert!(repl.tactic_intro("B".to_string()).is_ok());
    assert!(repl.tactic_intro("C".to_string()).is_ok());
    for i in 0..=target {
        println!("{}: {:?}", i, repl.arena.get(i));
    }
    println!("Target formula before intro H: {:?}", repl.arena.get(repl.active_state.as_ref().unwrap().goals[0].target));
    assert!(repl.tactic_intro("H".to_string()).is_ok());

    assert!(repl.tactic_destruct("H", "HA".to_string(), "HBC".to_string()).is_ok());
    
    // HBC is a disjunction, destructing it splits the goal into two.
    assert!(repl.tactic_destruct("HBC", "HB".to_string(), "HC".to_string()).is_ok());

    // Goal 1: x in A & x in B
    assert!(repl.tactic_left().is_ok());
    assert!(repl.tactic_split().is_ok());
    assert!(repl.tactic_exact("HA").is_ok());
    assert!(repl.tactic_exact("HB").is_ok());

    // Goal 2: x in A & x in C
    assert!(repl.tactic_right().is_ok());
    assert!(repl.tactic_split().is_ok());
    assert!(repl.tactic_exact("HA").is_ok());
    assert!(repl.tactic_exact("HC").is_ok());

    if let Some(state) = &repl.active_state {
        if state.goals.is_empty() {
            println!("Proof complete! QED.");
        } else {
            println!("Remaining goals: {}", state.goals.len());
        }
    }
}
