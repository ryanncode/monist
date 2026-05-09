use clap::{Parser as ClapParser, Subcommand};
use colored::*;
use monist_core::ast::{Atomic, Formula, FormulaArena, Var};
use monist_core::graph::{GraphArena, extract_constraints_aux};
use monist_core::smt::export_smt_lib;
use monist_parser::parser::Parser;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result as RlResult};
use serde::{Deserialize, Serialize};
use std::fs;

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

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Goal {
    context: Vec<(String, usize)>,
    target: usize,
}

#[derive(Serialize, Deserialize)]
struct Session {
    graph: GraphArena,
    axioms: Vec<String>,
    arena: FormulaArena,
    active_goals: Vec<Goal>,
    macros: std::collections::HashMap<String, (Vec<String>, usize)>,
}

impl Default for Session {
    fn default() -> Self {
        Self {
            graph: GraphArena::new(),
            axioms: Vec::new(),
            arena: FormulaArena::new(),
            active_goals: Vec::new(),
            macros: std::collections::HashMap::new(),
        }
    }
}

fn main() -> RlResult<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Repl) | None => run_repl(),
        Some(Commands::Verify { formula }) => {
            let mut arena = FormulaArena::new();
            let mut parser = Parser::with_macros(formula, &mut arena, None);
            let root_idx = parser.parse_formula();

            let constraints = extract_constraints_aux(&arena, root_idx, 0, false);
            let mut graph = GraphArena::from_constraints(&constraints);
            graph.collapse_scc_0_weight();

            match graph.bellman_ford() {
                Ok((_, actions)) => {
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
            let mut parser = Parser::with_macros(formula, &mut arena, None);
            let root_idx = parser.parse_formula();

            let constraints = extract_constraints_aux(&arena, root_idx, 0, false);
            let mut graph = GraphArena::from_constraints(&constraints);
            graph.collapse_scc_0_weight();

            let (trace, sc_actions, success_depths) = match graph.bellman_ford() {
                Ok((depths, actions)) => (None, actions, Some(depths)),
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
            let mut parser = Parser::with_macros(formula, &mut arena, None);
            let root_idx = parser.parse_formula();

            let constraints = extract_constraints_aux(&arena, root_idx, 0, false);
            let mut graph = GraphArena::from_constraints(&constraints);
            graph.collapse_scc_0_weight();

            if *export_smt {
                let (trace, sc_actions, success_depths) = match graph.bellman_ford() {
                    Ok((depths, actions)) => (None, actions, Some(depths)),
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
                match graph.bellman_ford() {
                    Ok((_, actions)) => {
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

    let mut session = Session::default();

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

fn process_repl_command(input: &str, session: &mut Session) {
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.is_empty() {
        return;
    }

    match parts[0] {
        "help" => {
            eprintln!("Commands:");
            eprintln!("  help                          - Show this help message");
            eprintln!("  exit                          - Exit the REPL");
            eprintln!("  save_session <file>           - Save current session to a JSON file");
            eprintln!("  load_session <file>           - Load a session from a JSON file");
            eprintln!("  eval <formula>                - Evaluate a formula");
            eprintln!("  step <formula>                - Step-by-step diagnostic evaluation");
            eprintln!("  assume <name> <formula>       - Add a named axiom");
            eprintln!("  theorem <name> <formula>      - Set a new goal to prove");
            eprintln!("  show_goal                     - Show the current goal state");
            eprintln!("  intro [name]                  - Introduce a hypothesis or variable");
            eprintln!(
                "  exact <name>                  - Close goal if it matches hypothesis exactly"
            );
            eprintln!("  split                         - Split a conjunction goal into two");
            eprintln!("  left                          - Prove left side of a disjunction");
            eprintln!("  right                         - Prove right side of a disjunction");
            eprintln!("  apply <name>                  - Apply a theorem/hypothesis");
            eprintln!("  destruct <name> [n1] [n2]     - Break down a hypothesis");
            eprintln!("  rewrite <name>                - Substitute variables using equality");
            eprintln!(
                "  deff <name>(<args>) := <formula> - Define a macro with Kosaraju SCC pre-flattening"
            );
            eprintln!("  cut <formula>                 - Introduce a formula as a sub-goal");
            eprintln!(
                "  focus_hyp <name>              - Pull a hypothesis to the top of the context"
            );
            eprintln!(
                "  defer                         - Skip the current goal and send it to the back"
            );
            eprintln!("  check_strat <formula>         - Run Bellman-Ford on raw geometry");
            eprintln!("  qed                           - Finish proof");
            eprintln!("  abort                         - Abort current proof");
        }

        "theorem" => {
            if parts.len() < 3 {
                eprintln!("{}", "Usage: theorem <name> <formula>".red());
                return;
            }
            let _name = parts[1].to_string();
            let formula = parts[2..].join(" ");
            let mut parser =
                Parser::with_macros(&formula, &mut session.arena, Some(&session.macros));
            let root_idx = parser.parse_formula();

            let goal = Goal {
                context: Vec::new(),
                target: root_idx,
            };
            session.active_goals.push(goal);
            eprintln!("[Goal Set] 1 unproven target.");
        }
        "show_goal" => {
            show_goal(session);
        }
        "qed" => {
            if session.active_goals.is_empty() {
                eprintln!("Proof accepted.");
            } else {
                eprintln!("There are still unproven goals.");
            }
        }
        "abort" => {
            session.active_goals.clear();
            eprintln!("Proof aborted.");
        }
        "rewrite" => {
            if parts.len() < 2 {
                eprintln!("{}", "Usage: rewrite <hyp_name>".red());
                return;
            }
            eprintln!("Rewriting..."); // Dummy rewrite implementation
        }
        "quit" => {
            std::process::exit(0);
        }

        "exit" => {
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
                    if let Err(e) = fs::write(filename, json) {
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
            match fs::read_to_string(filename) {
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
            if parts.len() < 2 {
                eprintln!("{}", "Usage: assume <axiom>".red());
                return;
            }
            let axiom = parts[1..].join(" ");
            session.axioms.push(axiom.clone());
            eprintln!("Assumed: {}", axiom.cyan());
        }
        "eval" => {
            if parts.len() < 2 {
                eprintln!("{}", "Usage: eval <formula>".red());
                return;
            }
            let formula = parts[1..].join(" ");

            let mut arena = FormulaArena::new();
            let mut parser = Parser::with_macros(&formula, &mut arena, None);
            let root_idx = parser.parse_formula();

            let constraints = extract_constraints_aux(&arena, root_idx, 0, false);
            let mut graph = GraphArena::from_constraints(&constraints);
            graph.collapse_scc_0_weight();

            // Merge with session graph? For now just evaluate independently
            match graph.bellman_ford() {
                Ok((_, _)) => eprintln!("{}", "Stratification successful.".green()),
                Err(e) => eprintln!("{}: {}", "Error".red(), e),
            }
        }
        "step" => {
            if parts.len() < 2 {
                eprintln!("{}", "Usage: step <formula>".red());
                return;
            }
            let formula = parts[1..].join(" ");

            let mut arena = FormulaArena::new();
            let mut parser = Parser::with_macros(&formula, &mut arena, None);
            let root_idx = parser.parse_formula();

            let constraints = extract_constraints_aux(&arena, root_idx, 0, false);
            let mut graph = GraphArena::from_constraints(&constraints);

            eprintln!("{}", "--- Extracting Constraints ---".yellow());
            for c in &constraints {
                eprintln!("{:?}", c);
            }

            eprintln!("{}", "--- Graph Nodes ---".yellow());
            for i in 0..graph.vars.len() {
                eprintln!("Node {}", i);
            }

            eprintln!("{}", "--- Graph Edges ---".yellow());
            for e in &graph.edges {
                if e.2 < 0 {
                    eprintln!("Edge {} -> {} weight {}", e.0, e.1, e.2.to_string().red());
                } else {
                    eprintln!("Edge {} -> {} weight {}", e.0, e.1, e.2);
                }
            }

            eprintln!("{}", "--- Collapsing SCC ---".yellow());
            graph.collapse_scc_0_weight();

            eprintln!("{}", "--- Running Bellman-Ford ---".yellow());
            match graph.bellman_ford() {
                Ok((_, _)) => eprintln!("{}", "Stratification successful.".green()),
                Err(e) => eprintln!("{}: {}", "Error".red(), e),
            }
        }
        "intro" => {
            let name = if parts.len() > 1 {
                parts[1].to_string()
            } else {
                "H".to_string()
            };
            if let Some(mut goal) = session.active_goals.pop() {
                let target = session.arena.get(goal.target).cloned();
                match target {
                    Some(Formula::Impl(l, r)) => {
                        goal.context.push((name, l));
                        goal.target = r;
                        session.active_goals.push(goal);
                        show_goal(session);
                    }
                    Some(Formula::Univ(_, _, inner)) => {
                        // For Univ, we just drop the binder and proceed with inner
                        goal.target = inner;
                        session.active_goals.push(goal);
                        show_goal(session);
                    }
                    _ => {
                        eprintln!(
                            "{}",
                            "Goal is not an implication or universal quantification.".red()
                        );
                        session.active_goals.push(goal);
                    }
                }
            } else {
                eprintln!("{}", "No active goals.".red());
            }
        }
        "exact" => {
            if parts.len() < 2 {
                eprintln!("{}", "Usage: exact <hyp_name>".red());
                return;
            }
            let name = parts[1].to_string();
            if let Some(goal) = session.active_goals.last() {
                if let Some((_, hyp_idx)) = goal.context.iter().find(|(n, _)| n == &name) {
                    if *hyp_idx == goal.target {
                        eprintln!("{}", "Goal closed!".green());
                        session.active_goals.pop();
                        if session.active_goals.is_empty() {
                            eprintln!("{}", "Proof complete!".green().bold());
                        } else {
                            show_goal(session);
                        }
                    } else {
                        eprintln!("{}", "Hypothesis does not exactly match the target.".red());
                    }
                } else {
                    eprintln!("{}", "Invalid hypothesis name.".red());
                }
            } else {
                eprintln!("{}", "No active goals.".red());
            }
        }
        "split" => {
            if let Some(mut goal) = session.active_goals.pop() {
                let target = session.arena.get(goal.target).cloned();
                match target {
                    Some(Formula::Conj(l, r)) => {
                        let mut goal2 = goal.clone();
                        goal.target = l;
                        goal2.target = r;
                        session.active_goals.push(goal2);
                        session.active_goals.push(goal);
                        show_goal(session);
                    }
                    _ => {
                        eprintln!("{}", "Goal is not a conjunction.".red());
                        session.active_goals.push(goal);
                    }
                }
            } else {
                eprintln!("{}", "No active goals.".red());
            }
        }
        "left" => {
            if let Some(mut goal) = session.active_goals.pop() {
                let target = session.arena.get(goal.target).cloned();
                match target {
                    Some(Formula::Disj(l, _)) => {
                        goal.target = l;
                        session.active_goals.push(goal);
                        show_goal(session);
                    }
                    _ => {
                        eprintln!("{}", "Goal is not a disjunction.".red());
                        session.active_goals.push(goal);
                    }
                }
            } else {
                eprintln!("{}", "No active goals.".red());
            }
        }
        "right" => {
            if let Some(mut goal) = session.active_goals.pop() {
                let target = session.arena.get(goal.target).cloned();
                match target {
                    Some(Formula::Disj(_, r)) => {
                        goal.target = r;
                        session.active_goals.push(goal);
                        show_goal(session);
                    }
                    _ => {
                        eprintln!("{}", "Goal is not a disjunction.".red());
                        session.active_goals.push(goal);
                    }
                }
            } else {
                eprintln!("{}", "No active goals.".red());
            }
        }
        "apply" => {
            if parts.len() < 2 {
                eprintln!("{}", "Usage: apply <hyp_name>".red());
                return;
            }
            let name = parts[1].to_string();
            if let Some(mut goal) = session.active_goals.pop() {
                if let Some(&(_, hyp_idx)) = goal.context.iter().find(|(n, _)| n == &name) {
                    let hyp_f = session.arena.get(hyp_idx).cloned();
                    match hyp_f {
                        Some(Formula::Impl(l, r)) if r == goal.target => {
                            goal.target = l;
                            session.active_goals.push(goal);
                            show_goal(session);
                        }
                        _ => {
                            eprintln!(
                                "{}",
                                "Hypothesis is not an implication matching the target.".red()
                            );
                            session.active_goals.push(goal);
                        }
                    }
                } else {
                    eprintln!("{}", "Invalid hypothesis name.".red());
                    session.active_goals.push(goal);
                }
            } else {
                eprintln!("{}", "No active goals.".red());
            }
        }
        "destruct" => {
            if parts.len() < 2 {
                eprintln!("{}", "Usage: destruct <hyp_name> [n1] [n2]".red());
                return;
            }
            let name = parts[1].to_string();
            let n1 = if parts.len() > 2 {
                parts[2].to_string()
            } else {
                format!("{}a", name)
            };
            let n2 = if parts.len() > 3 {
                parts[3].to_string()
            } else {
                format!("{}b", name)
            };

            if let Some(mut goal) = session.active_goals.pop() {
                if let Some(idx) = goal.context.iter().position(|(n, _)| n == &name) {
                    let (_, hyp_idx) = goal.context.remove(idx);
                    let hyp_f = session.arena.get(hyp_idx).cloned();
                    match hyp_f {
                        Some(Formula::Conj(l, r)) => {
                            goal.context.push((n1, l));
                            goal.context.push((n2, r));
                            session.active_goals.push(goal);
                            show_goal(session);
                        }
                        Some(Formula::Disj(l, r)) => {
                            let mut goal2 = goal.clone();
                            goal.context.push((n1, l));
                            goal2.context.push((n2, r));
                            session.active_goals.push(goal2);
                            session.active_goals.push(goal);
                            show_goal(session);
                        }
                        Some(Formula::Exist(_, _, inner)) => {
                            goal.context.push((n1, inner));
                            session.active_goals.push(goal);
                            show_goal(session);
                        }
                        _ => {
                            eprintln!("{}", "Hypothesis cannot be destructed.".red());
                            goal.context.insert(idx, (name, hyp_idx));
                            session.active_goals.push(goal);
                        }
                    }
                } else {
                    eprintln!("{}", "Invalid hypothesis name.".red());
                    session.active_goals.push(goal);
                }
            } else {
                eprintln!("{}", "No active goals.".red());
            }
        }
        "focus_hyp" => {
            if parts.len() < 2 {
                eprintln!("{}", "Usage: focus_hyp <hyp_name>".red());
                return;
            }
            let name = parts[1].to_string();
            if let Some(mut goal) = session.active_goals.pop() {
                if let Some(idx) = goal.context.iter().position(|(n, _)| n == &name) {
                    let hyp = goal.context.remove(idx);
                    goal.context.insert(0, hyp);
                    session.active_goals.push(goal);
                    show_goal(session);
                } else {
                    eprintln!("{}", "Invalid hypothesis name.".red());
                    session.active_goals.push(goal);
                }
            } else {
                eprintln!("{}", "No active goals.".red());
            }
        }
        "defer" => {
            if session.active_goals.len() > 1 {
                let goal = session.active_goals.pop().unwrap();
                session.active_goals.insert(0, goal);
                eprintln!("{}", "Goal deferred.".green());
                show_goal(session);
            } else if session.active_goals.len() == 1 {
                eprintln!("{}", "Only one active goal.".yellow());
            } else {
                eprintln!("{}", "No active goals.".red());
            }
        }
        "cut" => {
            if parts.len() < 2 {
                eprintln!("{}", "Usage: cut <formula>".red());
                return;
            }
            let formula_str = parts[1..].join(" ");
            let mut parser =
                Parser::with_macros(&formula_str, &mut session.arena, Some(&session.macros));
            let cut_idx = parser.parse_formula();

            if let Some(goal) = session.active_goals.pop() {
                let mut goal2 = goal.clone();
                goal2.context.push(("Cut".to_string(), cut_idx));

                let mut goal1 = goal.clone();
                goal1.target = cut_idx;

                session.active_goals.push(goal2);
                session.active_goals.push(goal1);
                show_goal(session);
            } else {
                eprintln!("{}", "No active goals.".red());
            }
        }
        "check_strat" => {
            if parts.len() < 2 {
                eprintln!("{}", "Usage: check_strat <formula>".red());
                return;
            }
            let formula = parts[1..].join(" ");

            let mut parser =
                Parser::with_macros(&formula, &mut session.arena, Some(&session.macros));
            let root_idx = parser.parse_formula();

            let constraints = extract_constraints_aux(&session.arena, root_idx, 0, false);
            let mut graph = GraphArena::from_constraints(&constraints);
            graph.collapse_scc_0_weight();

            match graph.bellman_ford() {
                Ok((_, _)) => eprintln!(
                    "{}",
                    "Stratification successful. Topologically sound.".green()
                ),
                Err(e) => eprintln!("{}: {}", "Error: Negative-weight cycle detected".red(), e),
            }
        }
        "deff" => {
            if parts.len() < 3 || !parts.contains(&":=") {
                eprintln!("{}", "Usage: deff <name>(<args>) := <formula>".red());
                return;
            }
            let eq_idx = parts.iter().position(|&x| x == ":=").unwrap();
            let sig_str = parts[1..eq_idx].join(" ");
            let formula_str = parts[eq_idx + 1..].join(" ");

            // parse signature: Name(A, B)
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

            let mut parser =
                Parser::with_macros(&formula_str, &mut session.arena, Some(&session.macros));
            let root_idx = parser.parse_formula();

            let constraints = extract_constraints_aux(&session.arena, root_idx, 0, false);
            let mut graph = GraphArena::from_constraints(&constraints);
            graph.collapse_scc_0_weight();

            session.macros.insert(name.clone(), (params, root_idx));
            eprintln!("Macro {} defined and SCC flattened.", name.cyan());
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

fn show_goal(session: &Session) {
    if let Some(goal) = session.active_goals.last() {
        eprintln!("{}", "--- Context ---".yellow());
        for hyp in goal.context.iter() {
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
}
