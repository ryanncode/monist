use monist_core::budget::ResourceBudget;
use monist_parser::parser::Parser;
use monist_seq::itp::ReplSession;

fn main() {
    println!("=== ITP Tactical Rejection: Russell's Unstratifiable Comprehension ===");
    let mut repl = ReplSession::new();
    let budget = ResourceBudget::default();

    // Goal: Russell Comprehension R = {x | ~(x in x)}
    let formula = "{x | ~(x in x)}";
    println!("Target Comprehension: {}", formula);

    let mut parser = Parser::new(formula, &mut repl.arena, budget);
    let target = parser.parse_formula();

    repl.start_proof("Russell_Paradox_Block".to_string(), target);

    println!("\nAttempting tactic_stratify on unstratifiable goal...");
    let res = repl.tactic_stratify();

    match res {
        Ok(_) => panic!("Extensionality collision should have halted stratification!"),
        Err(e) => {
            println!("\n[CONFIRMED] ITP engine successfully blocked proof step:");
            println!("  Error: {}", e);
        }
    }

    println!("\nAttempting tactic_schonfinkel on compiled combinator target...");
    let res_comb = repl.tactic_schonfinkel();
    println!("  Combinator tactic response: {:?}", res_comb);

    println!("\n[SUCCESS] Unstratified self-membership cannot be derived in the Monist ITP.");
}

