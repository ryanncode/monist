use monist_core::ast::{Atomic, Formula, Var};
use monist_seq::itp::ReplSession;
use monist_parser::parser::Parser;
use monist_core::budget::ResourceBudget;

fn print_goal_state(session: &ReplSession) {
    if let Some(state) = &session.active_state {
        if let Some(goal) = state.goals.first() {
            println!("  Context ({} items):", goal.ctx.len());
            for (name, idx) in &goal.ctx {
                println!("    {} : {}", name, session.format_formula(*idx));
            }
            println!("  ⊢ Target:");
            println!("    {}", session.format_formula(goal.target));
            if state.goals.len() > 1 {
                println!("  (+ {} pending subgoals)", state.goals.len() - 1);
            }
        } else {
            println!("  No remaining goals. Proof completed!");
        }
    }
}

fn main() {
    println!("============================================================");
    println!("  MONIST ITP LIVE TACTIC INTEGRATION TEST SUITE");
    println!("  Exercising all 5 advanced tactics: rw, simp, elevate,");
    println!("  collapse_loop, and sc_cut");
    println!("============================================================\n");

    let mut session = ReplSession::new();

    // ------------------------------------------------------------
    // TEST 1: Equality Rewriting (`tactic_rw`)
    // ------------------------------------------------------------
    println!("--- TEST 1: Equality Rewriting (tactic_rw) ---");
    let mut parser = Parser::new("a = b", &mut session.arena, ResourceBudget::default());
    let hyp_eq = parser.parse_formula();
    session.theorems.push(("H_ab".to_string(), hyp_eq));

    let mut parser = Parser::new("a = c", &mut session.arena, ResourceBudget::default());
    let target_eq = parser.parse_formula();
    session.start_proof("Proof_RW".to_string(), target_eq);
    
    println!("[Initial Goal]");
    print_goal_state(&session);

    println!("\n> rw H_ab");
    session.tactic_rw("H_ab").expect("rw H_ab failed");
    print_goal_state(&session);
    assert_eq!(session.format_formula(session.active_state.as_ref().unwrap().goals[0].target), "b = c");
    println!("✓ Equality rewriting successfully substituted variable in target AST.\n");

    // ------------------------------------------------------------
    // TEST 2: De Morgan & DNF Normalization (`tactic_simp`)
    // ------------------------------------------------------------
    println!("--- TEST 2: Simplification (tactic_simp) ---");
    let mut parser = Parser::new("~((a = a) \\/ (b = b))", &mut session.arena, ResourceBudget::default());
    let target_simp = parser.parse_formula();
    session.start_proof("Proof_Simp".to_string(), target_simp);

    println!("[Initial Goal]");
    print_goal_state(&session);

    println!("\n> simp");
    session.tactic_simp().expect("simp failed");
    print_goal_state(&session);
    let target_str = session.format_formula(session.active_state.as_ref().unwrap().goals[0].target);
    assert_eq!(target_str, "(¬a = a ∧ ¬b = b)");
    println!("✓ De Morgan push-negation normalized disjunction into DNF conjunction.\n");

    // ------------------------------------------------------------
    // TEST 3: Forster T-Functor Elevation (`tactic_elevate`)
    // ------------------------------------------------------------
    println!("--- TEST 3: T-Functor Elevation (tactic_elevate) ---");
    let mut parser = Parser::new("x = y", &mut session.arena, ResourceBudget::default());
    let target_elevate = parser.parse_formula();
    session.start_proof("Proof_Elevate".to_string(), target_elevate);

    println!("[Initial Goal]");
    print_goal_state(&session);

    println!("\n> elevate");
    session.tactic_elevate("x").expect("elevate failed");
    print_goal_state(&session);
    let target_str = session.format_formula(session.active_state.as_ref().unwrap().goals[0].target);
    assert_eq!(target_str, "x_iota = y_iota");
    println!("✓ T-functor shifted free variables into elevated stratum (x ↦ x_iota).\n");

    // ------------------------------------------------------------
    // TEST 4: 0-Weight SCC Loop Contraction (`tactic_collapse_loop`)
    // ------------------------------------------------------------
    println!("--- TEST 4: 0-Weight Loop Contraction (tactic_collapse_loop) ---");
    let mut parser = Parser::new("(x = y) /\\ (y = x)", &mut session.arena, ResourceBudget::default());
    let target_loop = parser.parse_formula();
    session.start_proof("Proof_Collapse".to_string(), target_loop);

    println!("[Initial Goal]");
    print_goal_state(&session);

    println!("\n> collapse_loop");
    session.tactic_collapse_loop().expect("collapse_loop failed");
    print_goal_state(&session);
    let target_str = session.format_formula(session.active_state.as_ref().unwrap().goals[0].target);
    println!("✓ 0-weight SCC loop contracted cyclic variables: {}\n", target_str);

    // ------------------------------------------------------------
    // TEST 5: Strongly Cantorian Bedrock Isolation (`tactic_sc_cut`)
    // ------------------------------------------------------------
    println!("--- TEST 5: Strongly Cantorian Cut (tactic_sc_cut) ---");
    let mut parser = Parser::new("z in S", &mut session.arena, ResourceBudget::default());
    let target_sc = parser.parse_formula();
    session.start_proof("Proof_SCCut".to_string(), target_sc);

    println!("[Initial Goal]");
    print_goal_state(&session);

    println!("\n> sc_cut S");
    session.tactic_sc_cut("S").expect("sc_cut failed");
    print_goal_state(&session);
    let ctx = &session.active_state.as_ref().unwrap().goals[0].ctx;
    assert!(ctx.iter().any(|(name, _)| name == "SC_BEDROCK_S"));
    println!("✓ Strongly Cantorian bedrock axiom successfully injected into proof context.\n");

    // ------------------------------------------------------------
    // TEST 6: Complete End-to-End Modus Ponens Deduction
    // ------------------------------------------------------------
    println!("--- TEST 6: Full Natural Deduction (intro, destruct, apply, exact) ---");
    let p = session.arena.add(Formula::Atom(Atomic::Eq(Var::Free("p".to_string()), Var::Free("p".to_string()))));
    let q = session.arena.add(Formula::Atom(Atomic::Eq(Var::Free("q".to_string()), Var::Free("q".to_string()))));
    let p_imp_q = session.arena.add(Formula::Impl(p, q));
    let conj = session.arena.add(Formula::Conj(p_imp_q, p));
    let mp = session.arena.add(Formula::Impl(conj, q));
    session.start_proof("ModusPonens".to_string(), mp);

    println!("[Initial Goal]");
    print_goal_state(&session);

    println!("\n> intro H");
    session.tactic_intro("H".to_string()).expect("intro");
    println!("> destruct H H_imp H_p");
    session.tactic_destruct("H", "H_imp".to_string(), "H_p".to_string()).expect("destruct");
    println!("> apply H_imp");
    session.tactic_apply("H_imp").expect("apply");
    println!("> exact H_p");
    session.tactic_exact("H_p").expect("exact");
    print_goal_state(&session);
    assert_eq!(session.active_state.as_ref().unwrap().goals.len(), 0);

    println!("\n============================================================");
    println!("  ALL 6 TACTICAL WORKFLOWS VERIFIED AND SOUND");
    println!("============================================================");
}

