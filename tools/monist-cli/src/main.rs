use clap::{Parser as ClapParser, Subcommand};
use colored::*;
use monist_core::ast::{Atomic, Formula, FormulaArena, Var};
use monist_core::graph::{GraphArena, extract_constraints_aux};
use monist_core::smt::export_smt_lib;
use monist_parser::parser::Parser;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result as RlResult};
use monist_seq::itp::ReplSession;

mod demos;

#[derive(ClapParser)]
#[command(name = "monist-cli")]
#[command(about = "Monist Engine CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start an interactive REPL session
    Repl,
    /// Evaluate a single formula
    Eval {
        /// The formula to evaluate
        formula: String,
        /// Export to SMT-LIB format
        #[arg(long)]
        export_smt: bool,
    },
    /// Verify a single formula without entering REPL
    Verify { formula: String },
    /// Export a StratificationWitness in SMT-LIB format
    ExportSmt { formula: String },
    /// Run visual demonstrations
    Demo {
        #[command(subcommand)]
        action: DemoAction,
    },
}

#[derive(Subcommand)]
enum DemoAction {
    /// Holographic Sieve Visualizer
    Holographic,
    /// Agentic Reflection Topology Visualizer
    Agentic,
}


fn main() -> RlResult<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Repl) | None => run_repl(),
        Some(Commands::Verify { formula }) => {
            let mut arena = FormulaArena::new();
            let mut parser = Parser::with_macros(formula, &mut arena, None, monist_core::budget::ResourceBudget::default());
            let root_idx = parser.parse_formula();

            let constraints = extract_constraints_aux(&arena, root_idx, 0, false, &monist_core::budget::ResourceBudget::default(), &mut 0);
            let mut graph = GraphArena::from_constraints(&constraints);
            graph.collapse_scc_0_weight();

            match graph.evaluate_topology() {
                Ok((_, actions, _, _)) => {
                    eprintln!("{}", "Stratification successful.".green());
                    for action in actions {
                        eprintln!("{}", action.cyan());
                    }
                }
                Err(e) => eprintln!("{}: {}", "Error".red(), e),
            }
            Ok(())
        }
        Some(Commands::ExportSmt { formula }) => {
            let mut arena = FormulaArena::new();
            let mut parser = Parser::with_macros(formula, &mut arena, None, monist_core::budget::ResourceBudget::default());
            let root_idx = parser.parse_formula();

            let constraints = extract_constraints_aux(&arena, root_idx, 0, false, &monist_core::budget::ResourceBudget::default(), &mut 0);
            let mut graph = GraphArena::from_constraints(&constraints);
            graph.collapse_scc_0_weight();

            let (trace, sc_actions, success_depths) = match graph.evaluate_topology() {
                Ok((depths, actions, _, _)) => (None, actions, Some(depths)),
                Err(e) => (Some(e), Vec::new(), None),
            };

            let smt_output = export_smt_lib(
                &graph,
                "cli_input",
                trace.as_deref(),
                &sc_actions,
                success_depths.as_deref(),
            );
            println!("{}", smt_output);
            Ok(())
        }
        Some(Commands::Eval {
            formula,
            export_smt,
        }) => {
            let mut arena = FormulaArena::new();
            let mut parser = Parser::with_macros(formula, &mut arena, None, monist_core::budget::ResourceBudget::default());
            let root_idx = parser.parse_formula();

            let constraints = extract_constraints_aux(&arena, root_idx, 0, false, &monist_core::budget::ResourceBudget::default(), &mut 0);
            let mut graph = GraphArena::from_constraints(&constraints);
            graph.collapse_scc_0_weight();

            if *export_smt {
                let (trace, sc_actions, success_depths) = match graph.evaluate_topology() {
                    Ok((depths, actions, _, _)) => (None, actions, Some(depths)),
                    Err(e) => (Some(e), Vec::new(), None),
                };

                let smt_output = export_smt_lib(
                    &graph,
                    "cli_input",
                    trace.as_deref(),
                    &sc_actions,
                    success_depths.as_deref(),
                );
                println!("{}", smt_output);
            } else {
                match graph.evaluate_topology() {
                    Ok((_, actions, _, _)) => {
                        eprintln!("{}", "Stratification successful.".green());
                        for action in actions {
                            eprintln!("{}", action.cyan());
                        }
                    }
                    Err(e) => eprintln!("{}: {}", "Error".red(), e),
                }
            }
            Ok(())
        }
        Some(Commands::Demo { action }) => {
            match action {
                DemoAction::Holographic => demos::run_holographic_demo(),
                DemoAction::Agentic => demos::run_agentic_demo(),
            }
            Ok(())
        }
    }
}

fn run_repl() -> RlResult<()> {
    eprintln!("{}", "Welcome to Monist Engine REPL.".cyan().bold());
    eprintln!("Type 'help' for a list of commands, or 'exit' to quit.");

    let mut rl = DefaultEditor::new()?;
    let _ = rl.load_history("history.txt");

    let mut session = ReplSession::new();

    loop {
        let readline = rl.readline("monist> ");
        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                process_repl_command(&line, &mut session);
            }
            Err(ReadlineError::Interrupted) => {
                eprintln!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                eprintln!("CTRL-D");
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }

    let _ = rl.save_history("history.txt");
    Ok(())
}

fn process_repl_command(input: &str, session: &mut ReplSession) {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.is_empty() {
        return;
    }

    match parts[0] {
        "help" => {
            eprintln!("{}", "Session & Proof Management:".green().bold());
            eprintln!("  help                          - Show this help message");
            eprintln!("  exit | quit                   - Exit the REPL");
            eprintln!("  save_session <file>           - Save current session to a JSON file");
            eprintln!("  load_session <file>           - Load a session from a JSON file");
            eprintln!("  theorem <name> <formula>      - Set a new goal to prove");
            eprintln!("  show_goal                     - Show the current goal state");
            eprintln!("  qed                           - Finish proof");
            eprintln!("  abort                         - Abort current proof");
            
            eprintln!("\n{}", "Global Commands:".green().bold());
            eprintln!("  eval <formula>                - Evaluate a formula");
            eprintln!("  check_strat <formula>         - Run Bellman-Ford on raw geometry");
            eprintln!("  assume <name> <formula>       - Add a named axiom");
            eprintln!("  deff <name>(<args>) := <form> - Define a macro");

            eprintln!("\n{}", "Logical Tactics:".green().bold());
            eprintln!("  intro [name]                  - Introduce a hypothesis or variable");
            eprintln!("  exact <name>                  - Close goal if it matches hypothesis exactly");
            eprintln!("  apply <name>                  - Apply a theorem/hypothesis");
            eprintln!("  split                         - Split a conjunction goal into two");
            eprintln!("  left                          - Prove left side of a disjunction");
            eprintln!("  right                         - Prove right side of a disjunction");
            eprintln!("  destruct <name> [n1] [n2]     - Break down a hypothesis");
            eprintln!("  cut <formula>                 - Introduce a formula as a sub-goal");
            eprintln!("  have <name> <formula>         - Prove a sub-goal and add to context");
            eprintln!("  focus_hyp <name>              - Pull a hypothesis to top of context");
            eprintln!("  defer                         - Skip current goal to back of queue");
            
            eprintln!("\n{}", "Topological Tactics:".green().bold());
            eprintln!("  stratify                      - Weak stratification topological check");
            eprintln!("  refl                          - DAG topological equivalence check");
            eprintln!("  schonfinkel                   - SKI combinator extraction");
            eprintln!("  step [formula]                - Execute geometric evaluation on target or formula");
            
            // Mocked / WIP tactics:
            // eprintln!("\n{}", "WIP Tactics:".yellow().bold());
            // eprintln!("  simp                          - Simplify current goal");
            // eprintln!("  rw <formula>                  - Rewrite target");
            // eprintln!("  elevate <name>                - T-Functor elevation");
            // eprintln!("  collapse_loop                 - Contract SCC topologies");
            // eprintln!("  sc_cut <formula>              - Quarantine sub-goal in Strongly Cantorian boundary");
        }
        "theorem" => {
            if parts.len() < 3 {
                eprintln!("{}", "Usage: theorem <name> <formula>".red());
                return;
            }
            let name = parts[1].to_string();
            let formula = parts[2..].join(" ");
            let mut parser = Parser::with_macros(&formula, &mut session.arena, Some(&session.macros), monist_core::budget::ResourceBudget::default());
            let root_idx = parser.parse_formula();
            session.start_proof(name, root_idx);
            eprintln!("[Goal Set] 1 unproven target.");
            show_goal(session);
        }
        "show_goal" => {
            show_goal(session);
        }
        "qed" => {
            if let Some(state) = &session.active_state {
                if state.goals.is_empty() {
                    eprintln!("Proof accepted.");
                } else {
                    eprintln!("There are still unproven goals.");
                }
            } else {
                eprintln!("No active proof.");
            }
        }
        "abort" => {
            session.active_state = None;
            eprintln!("Proof aborted.");
        }
        "quit" | "exit" => {
            std::process::exit(0);
        }
        "save_session" => {
            if parts.len() < 2 {
                eprintln!("{}", "Usage: save_session <file>".red());
                return;
            }
            let filename = parts[1];
            match serde_json::to_string_pretty(session) {
                Ok(json) => {
                    if let Err(e) = std::fs::write(filename, json) {
                        eprintln!("{}: {}", "Failed to save session".red(), e);
                    } else {
                        eprintln!("Session saved to {}", filename.green());
                    }
                }
                Err(e) => eprintln!("{}: {}", "Failed to serialize session".red(), e),
            }
        }
        "load_session" => {
            if parts.len() < 2 {
                eprintln!("{}", "Usage: load_session <file>".red());
                return;
            }
            let filename = parts[1];
            match std::fs::read_to_string(filename) {
                Ok(json) => match serde_json::from_str(&json) {
                    Ok(loaded_session) => {
                        *session = loaded_session;
                        eprintln!("Session loaded from {}", filename.green());
                    }
                    Err(e) => eprintln!("{}: {}", "Failed to deserialize session".red(), e),
                },
                Err(e) => eprintln!("{}: {}", "Failed to load session".red(), e),
            }
        }
        "assume" => {
            if parts.len() < 3 {
                eprintln!("{}", "Usage: assume <name> <formula>".red());
                return;
            }
            let name = parts[1].to_string();
            let formula = parts[2..].join(" ");
            let mut parser = Parser::with_macros(&formula, &mut session.arena, Some(&session.macros), monist_core::budget::ResourceBudget::default());
            let root_idx = parser.parse_formula();
            session.theorems.push((name.clone(), root_idx));
            eprintln!("Assumed: {}", name.cyan());
        }
        "eval" => {
            if parts.len() < 2 {
                eprintln!("{}", "Usage: eval <formula>".red());
                return;
            }
            let formula = parts[1..].join(" ");
            let mut parser = Parser::with_macros(&formula, &mut session.arena, Some(&session.macros), monist_core::budget::ResourceBudget::default());
            let root_idx = parser.parse_formula();
            let constraints = extract_constraints_aux(&session.arena, root_idx, 0, false, &monist_core::budget::ResourceBudget::default(), &mut 0);
            let mut graph = GraphArena::from_constraints(&constraints);
            graph.collapse_scc_0_weight();
            match graph.evaluate_topology() {
                Ok((_, _, _, _)) => eprintln!("{}", "Stratification successful.".green()),
                Err(e) => eprintln!("{}: {}", "Error".red(), e),
            }
        }
        "step" => {
            if parts.len() == 1 {
                if let Err(e) = session.tactic_step() {
                    eprintln!("{}: {}", "Error".red(), e);
                } else {
                    show_goal(session);
                }
            } else {
                let formula = parts[1..].join(" ");
                let mut parser = Parser::with_macros(&formula, &mut session.arena, Some(&session.macros), monist_core::budget::ResourceBudget::default());
                let root_idx = parser.parse_formula();
                let constraints = extract_constraints_aux(&session.arena, root_idx, 0, false, &monist_core::budget::ResourceBudget::default(), &mut 0);
                let mut graph = GraphArena::from_constraints(&constraints);
                eprintln!("{}", "--- Extracting Constraints ---".yellow());
                for c in &constraints { eprintln!("{:?}", c); }
                eprintln!("{}", "--- Graph Nodes ---".yellow());
                for i in 0..graph.vars.len() { eprintln!("Node {}", i); }
                eprintln!("{}", "--- Graph Edges ---".yellow());
                for e in &graph.edges {
                    if e.2 < 0 { eprintln!("Edge {} -> {} weight {}", e.0, e.1, e.2.to_string().red()); } 
                    else { eprintln!("Edge {} -> {} weight {}", e.0, e.1, e.2); }
                }
                eprintln!("{}", "--- Collapsing SCC ---".yellow());
                graph.collapse_scc_0_weight();
                eprintln!("{}", "--- Running Bellman-Ford ---".yellow());
                match graph.evaluate_topology() {
                    Ok((_, _, _, _)) => eprintln!("{}", "Stratification successful.".green()),
                    Err(e) => eprintln!("{}: {}", "Error".red(), e),
                }
            }
        }
        "intro" => {
            let name = parts.get(1).cloned().unwrap_or("H").to_string();
            if let Err(e) = session.tactic_intro(name) { eprintln!("{}: {}", "Error".red(), e); } else { show_goal(session); }
        }
        "exact" => {
            let name = parts.get(1).cloned().unwrap_or("").to_string();
            if let Err(e) = session.tactic_exact(&name) { eprintln!("{}: {}", "Error".red(), e); } else { show_goal(session); }
        }
        "apply" => {
            let name = parts.get(1).cloned().unwrap_or("").to_string();
            if let Err(e) = session.tactic_apply(&name) { eprintln!("{}: {}", "Error".red(), e); } else { show_goal(session); }
        }
        "split" => {
            if let Err(e) = session.tactic_split() { eprintln!("{}: {}", "Error".red(), e); } else { show_goal(session); }
        }
        "left" => {
            if let Err(e) = session.tactic_left() { eprintln!("{}: {}", "Error".red(), e); } else { show_goal(session); }
        }
        "right" => {
            if let Err(e) = session.tactic_right() { eprintln!("{}: {}", "Error".red(), e); } else { show_goal(session); }
        }
        "destruct" => {
            let name = parts.get(1).cloned().unwrap_or("").to_string();
            let n1 = parts.get(2).cloned().unwrap_or("").to_string();
            let n2 = parts.get(3).cloned().unwrap_or("").to_string();
            if let Err(e) = session.tactic_destruct(&name, n1, n2) { eprintln!("{}: {}", "Error".red(), e); } else { show_goal(session); }
        }
        "cut" => {
            let formula = parts[1..].join(" ");
            if let Err(e) = session.tactic_cut(&formula) { eprintln!("{}: {}", "Error".red(), e); } else { show_goal(session); }
        }
        "stratify" => {
            if let Err(e) = session.tactic_stratify() { eprintln!("{}: {}", "Error".red(), e); } else { show_goal(session); }
        }
        "refl" => {
            if let Err(e) = session.tactic_refl() { eprintln!("{}: {}", "Error".red(), e); } else { show_goal(session); }
        }
        "have" => {
            if parts.len() < 3 {
                eprintln!("{}", "Usage: have <name> <formula>".red());
                return;
            }
            let name = parts[1].to_string();
            let formula = parts[2..].join(" ");
            if let Err(e) = session.tactic_have(&name, &formula) { eprintln!("{}: {}", "Error".red(), e); } else { show_goal(session); }
        }
        "schonfinkel" => {
            if let Err(e) = session.tactic_schonfinkel() { eprintln!("{}: {}", "Error".red(), e); } else { show_goal(session); }
        }
        "simp" => {
            if let Err(e) = session.tactic_simp() { eprintln!("{}: {}", "Error".red(), e); } else { show_goal(session); }
        }
        "rw" => {
            let formula = parts[1..].join(" ");
            if let Err(e) = session.tactic_rw(&formula) { eprintln!("{}: {}", "Error".red(), e); } else { show_goal(session); }
        }
        "focus_hyp" => {
            let name = parts.get(1).cloned().unwrap_or("").to_string();
            if let Err(e) = session.tactic_focus_hyp(&name) { eprintln!("{}: {}", "Error".red(), e); } else { show_goal(session); }
        }
        "defer" => {
            if let Err(e) = session.tactic_defer() { eprintln!("{}: {}", "Error".red(), e); } else { show_goal(session); }
        }
        "elevate" => {
            let name = parts.get(1).cloned().unwrap_or("").to_string();
            if let Err(e) = session.tactic_elevate(&name) { eprintln!("{}: {}", "Error".red(), e); } else { show_goal(session); }
        }
        "collapse_loop" => {
            if let Err(e) = session.tactic_collapse_loop() { eprintln!("{}: {}", "Error".red(), e); } else { show_goal(session); }
        }
        "deff" => {
            if parts.len() < 3 || !parts.contains(&":=") {
                eprintln!("{}", "Usage: deff <name>(<args>) := <formula>".red());
                return;
            }
            let eq_idx = parts.iter().position(|&x| x == ":=").unwrap();
            let sig_str = parts[1..eq_idx].join(" ");
            let formula_str = parts[eq_idx + 1..].join(" ");
            let sig_str = sig_str.replace(" ", "");
            let open_paren = sig_str.find('(');
            let close_paren = sig_str.find(')');
            let name;
            let mut params = Vec::new();
            if let (Some(op), Some(cp)) = (open_paren, close_paren) {
                name = sig_str[..op].to_string();
                let params_str = &sig_str[op + 1..cp];
                if !params_str.is_empty() {
                    params = params_str.split(',').map(|s| s.to_string()).collect();
                }
            } else {
                name = sig_str;
            }
            if let Err(e) = session.define_macro(name.clone(), params, &formula_str) {
                eprintln!("{}: {}", "Error".red(), e);
            } else {
                eprintln!("Macro {} defined and SCC flattened.", name.cyan());
            }
        }
        "check_strat" => {
            if parts.len() < 2 {
                eprintln!("{}", "Usage: check_strat <formula>".red());
                return;
            }
            let formula = parts[1..].join(" ");
            let mut parser = Parser::with_macros(&formula, &mut session.arena, Some(&session.macros), monist_core::budget::ResourceBudget::default());
            let root_idx = parser.parse_formula();
            let constraints = extract_constraints_aux(&session.arena, root_idx, 0, false, &monist_core::budget::ResourceBudget::default(), &mut 0);
            let mut graph = GraphArena::from_constraints(&constraints);
            graph.collapse_scc_0_weight();
            match graph.evaluate_topology() {
                Ok((_, _, _, _)) => eprintln!("{}", "Stratification successful. Topologically sound.".green()),
                Err(e) => eprintln!("{}: {}", "Error: Negative-weight cycle detected".red(), e),
            }
        }
        _ => {
            eprintln!("{}: Unknown command '{}'", "Error".red(), parts[0]);
        }
    }
}

fn format_formula(arena: &FormulaArena, idx: usize, show_tags: bool) -> String {
    let formula = match arena.get(idx) {
        Some(f) => f,
        None => return format!("<?{}>", idx),
    };
    match formula {
        Formula::Atom(Atomic::Eq(v1, v2)) => format!(
            "{} = {}",
            format_var(v1, show_tags),
            format_var(v2, show_tags)
        ),
        Formula::Atom(Atomic::Mem(v1, v2)) => format!(
            "{} in {}",
            format_var(v1, show_tags),
            format_var(v2, show_tags)
        ),
        Formula::Atom(a) => format!("{:?}", a),
        Formula::Neg(i) => format!("~{}", format_formula(arena, *i, show_tags)),
        Formula::Conj(l, r) => format!(
            "({} /\\ {})",
            format_formula(arena, *l, show_tags),
            format_formula(arena, *r, show_tags)
        ),
        Formula::Disj(l, r) => format!(
            "({} \\/ {})",
            format_formula(arena, *l, show_tags),
            format_formula(arena, *r, show_tags)
        ),
        Formula::Impl(l, r) => format!(
            "({} -> {})",
            format_formula(arena, *l, show_tags),
            format_formula(arena, *r, show_tags)
        ),
        Formula::Univ(_, var, inner) => format!(
            "forall {}. {}",
            var,
            format_formula(arena, *inner, show_tags)
        ),
        Formula::Exist(_, var, inner) => format!(
            "exists {}. {}",
            var,
            format_formula(arena, *inner, show_tags)
        ),
        Formula::Comp(_, var, inner) => format!(
            "{{ {} | {} }}",
            var,
            format_formula(arena, *inner, show_tags)
        ),
    }
}

fn format_var(v: &Var, show_tags: bool) -> String {
    match v {
        Var::Free(name) => {
            if !show_tags && name.contains('@') {
                name.split('@').next().unwrap_or(name).to_string()
            } else {
                name.clone()
            }
        }
        Var::Bound(idx) => {
            if show_tags {
                format!("^{}", idx)
            } else {
                format!("v{}", idx)
            }
        }
    }
}

fn show_goal(session: &ReplSession) {
    if let Some(state) = &session.active_state {
        if let Some(goal) = state.goals.first() {
            eprintln!("{}", "--- Context ---".yellow());
            for hyp in goal.ctx.iter() {
                eprintln!(
                    "{}: {}",
                    hyp.0,
                    format_formula(&session.arena, hyp.1, false)
                );
            }
            eprintln!("{}", "--- Target ---".yellow());
            eprintln!(
                "{}",
                format_formula(&session.arena, goal.target, false)
                    .cyan()
                    .bold()
            );
        } else {
            eprintln!("No active goals.");
        }
    } else {
        eprintln!("No active proof.");
    }
}
