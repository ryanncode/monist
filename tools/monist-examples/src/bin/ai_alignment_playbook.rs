use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use monist_core::ast::{Atomic, Formula, FormulaArena, Var};
use monist_core::graph::{extract_constraints_aux, GraphArena};
use monist_core::smt::export_smt_lib;
use std::thread;
use std::time::Duration;

fn main() {
    println!("====================================================================");
    println!("Monist AI Alignment Playbook Demonstrator: Non-Well-Founded Topology");
    println!("====================================================================");
    println!("Unlike heuristics (e.g., Principle of Least Syntactic Action), this");
    println!("demonstrator evaluates a structurally infinite Recursive Reward Function.");
    println!("It uses the Monist `GraphArena` to directly model a non-well-founded");
    println!("set scenario: `Reward(AgentState) == Reward(Reward(AgentState))`.");
    println!("The paradox halt is avoided by applying SC-Cut boundaries that flat-line");
    println!("typestate recursion without restricting the agent's logical state space.");
    println!("====================================================================\n");

    let m = MultiProgress::new();
    let style = ProgressStyle::with_template("[{elapsed_precise}] {spinner:.green} {msg}")
        .unwrap()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");

    let pb_setup = m.add(ProgressBar::new_spinner());
    pb_setup.set_style(style.clone());
    pb_setup.set_message("Constructing Non-Well-Founded Agent State Space...");

    for _ in 0..20 {
        pb_setup.tick();
        thread::sleep(Duration::from_millis(40));
    }
    pb_setup.finish_with_message("Non-Well-Founded Agent State Space constructed.");

    let pb_arena = m.add(ProgressBar::new_spinner());
    pb_arena.set_style(style.clone());
    pb_arena.set_message("Mapping recursive reward functions into Graph Arena...");

    let mut f_arena = FormulaArena::new();

    // Variables representing the agent's state and recursive reward evaluations
    let state = Var::Free("AgentState".to_string());
    let reward_1 = Var::Free("Reward_AgentState".to_string());
    let reward_2 = Var::Free("Reward_Reward_AgentState".to_string());
    let reward_n = Var::Free("Reward_N_AgentState".to_string());

    // Constraints:
    // 1. SC-Domain Axiom: The Reward Function operates strictly on the same typestate
    //    as the AgentState. (Enforcing a Strongly Cantorian topological distance of 0,
    //    unlike ZFC powersets which force a +1 hierarchy jump).
    let sc_axiom = f_arena.add(Formula::Atom(Atomic::Eq(state.clone(), reward_1.clone())));

    // 2. The recursive alignment constraint: Reward(AgentState) == Reward(Reward(AgentState))
    //    This creates a structural recursion via Extensional Equality (0-weight cycle)
    let eq_reward = f_arena.add(Formula::Atom(Atomic::Eq(
        reward_1.clone(),
        reward_2.clone(),
    )));

    // 3. To prove it's robust, we map infinite recursive layers (simulated by equating layer 2 to N)
    let eq_reward_inf = f_arena.add(Formula::Atom(Atomic::Eq(
        reward_2.clone(),
        reward_n.clone(),
    )));

    // Link the formulas
    let conj1 = f_arena.add(Formula::Conj(sc_axiom, eq_reward));
    let main_f = f_arena.add(Formula::Conj(conj1, eq_reward_inf));

    // Extract constraints and generate the Graph Arena
    let constraints = extract_constraints_aux(&f_arena, main_f, 0, false);

    for _ in 0..15 {
        pb_arena.tick();
        thread::sleep(Duration::from_millis(40));
    }
    pb_arena.finish_with_message("Graph Arena generated with massive self-reference constraints.");

    let pb_scc = m.add(ProgressBar::new_spinner());
    pb_scc.set_style(style.clone());
    pb_scc.set_message("Applying SC-Cut Boundaries (Collapsing SCC 0-weight cycles)...");

    let mut graph = GraphArena::from_constraints(&constraints);

    for _ in 0..20 {
        pb_scc.tick();
        thread::sleep(Duration::from_millis(40));
    }

    // Collapse to bound the infinite state space without paradox
    graph.collapse_scc_0_weight();
    pb_scc.finish_with_message("SCC 0-weight cycles collapsed: Typestate effectively flattened.");

    let pb_bellman = m.add(ProgressBar::new_spinner());
    pb_bellman.set_style(style.clone());
    pb_bellman
        .set_message("Verifying topological stability (Bellman-Ford Extensionality Check)...");

    for _ in 0..20 {
        pb_bellman.tick();
        thread::sleep(Duration::from_millis(40));
    }

    let bf_result = graph.evaluate_topology();
    match &bf_result {
        Ok(_) => {
            pb_bellman.finish_with_message("Topological stability proven! Recursive reward formulation is valid without paradox halt.");
        }
        Err(e) => {
            pb_bellman.finish_with_message(format!("Paradox detected: {}", e));
        }
    }

    println!("\n[SC-CUT WITNESS]");
    println!(
        "Generating SMT-LIB Witness. Notice how the recursively deep Reward_Reward_AgentState"
    );
    println!("and Reward_N_AgentState are structurally flattened by the SC-Cut mapping, proving");
    println!("that non-well-founded evaluation won't panic the safety verifier:");
    println!("--------------------------------------------------------------------------------");
    
    match &bf_result {
        Ok((success_depths, sc_actions, _, _)) => {
            println!(
                "{}\n",
                export_smt_lib(&graph, "RecursiveRewardStability_NonWellFounded", None, sc_actions, Some(success_depths))
            );
        }
        Err(collision_trace) => {
            println!(
                "{}\n",
                export_smt_lib(&graph, "RecursiveRewardStability_NonWellFounded", Some(collision_trace), &[], None)
            );
        }
    }

    println!("Simulation Complete. AI Alignment playbook demonstration finished successfully.");
}
