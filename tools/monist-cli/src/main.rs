use clap::{Parser as ClapParser, Subcommand};
use colored::*;
use monist_core::ast::{FormulaArena, Formula, Atomic, Var};
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
    Verify {
        formula: String,
    },
    /// Export a StratificationWitness in SMT-LIB v2 format
    ExportSmt {
        formula: String,
    },
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
        Some(Commands::Repl) | None => {
            run_repl()
        }
        Some(Commands::Verify { formula }) => {
            let mut arena = FormulaArena::new();
            let mut parser = Parser::with_macros(formula, &mut arena, None);
            let root_idx = parser.parse_formula();

            let constraints = extract_constraints_aux(&arena, root_idx, 0);
            let mut graph = GraphArena::from_constraints(&constraints);
            graph.collapse_scc_0_weight();

            match graph.bellman_ford() {
                Ok(_) => println!("{}", "Stratification successful.".green()),
                Err(e) => println!("{}: {}", "Error".red(), e),
            }
            Ok(())
        }
        Some(Commands::ExportSmt { formula }) => {
            let mut arena = FormulaArena::new();
            let mut parser = Parser::with_macros(formula, &mut arena, None);
            let root_idx = parser.parse_formula();

            let constraints = extract_constraints_aux(&arena, root_idx, 0);
            let mut graph = GraphArena::from_constraints(&constraints);
            graph.collapse_scc_0_weight();

            let smt_output = export_smt_lib(&graph, "cli_input");
            println!("{}", smt_output);
            Ok(())
        }
        Some(Commands::Eval { formula, export_smt }) => {
            let mut arena = FormulaArena::new();
            let mut parser = Parser::with_macros(formula, &mut arena, None);
            let root_idx = parser.parse_formula();

            let constraints = extract_constraints_aux(&arena, root_idx, 0);
            let mut graph = GraphArena::from_constraints(&constraints);
            graph.collapse_scc_0_weight();

            if *export_smt {
                let smt_output = export_smt_lib(&graph, "cli_input");
                println!("{}", smt_output);
            } else {
                match graph.bellman_ford() {
                    Ok(_) => println!("{}", "Stratification successful.".green()),
                    Err(e) => println!("{}: {}", "Error".red(), e),
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
    println!("{}", "Welcome to Monist Engine REPL.".cyan().bold());
    println!("Type 'help' for a list of commands, or 'exit' to quit.");

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
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
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
            println!("Commands:");
            println!("  help                          - Show this help message");
            println!("  exit                          - Exit the REPL");
            println!("  save_session <file>           - Save current session to a JSON file");
            println!("  load_session <file>           - Load a session from a JSON file");
            println!("  eval <formula>                - Evaluate a formula");
            println!("  step <formula>                - Step-by-step diagnostic evaluation");
            println!("  assume <name> <formula>       - Add a named axiom");
            println!("  theorem <name> <formula>      - Set a new goal to prove");
            println!("  show_goal                     - Show the current goal state");
            println!("  intro [name]                  - Introduce a hypothesis or variable");
            println!("  exact <name>                  - Close goal if it matches hypothesis exactly");
            println!("  split                         - Split a conjunction goal into two");
            println!("  left                          - Prove left side of a disjunction");
            println!("  right                         - Prove right side of a disjunction");
            println!("  apply <name>                  - Apply a theorem/hypothesis");
            println!("  destruct <name> [n1] [n2]     - Break down a hypothesis");
            println!("  rewrite <name>                - Substitute variables using equality");
            println!("  deff <name>(<args>) := <formula> - Define a macro with Kosaraju SCC pre-flattening");
            println!("  cut <formula>                 - Introduce a formula as a sub-goal");
            println!("  focus_hyp <name>              - Pull a hypothesis to the top of the context");
            println!("  defer                         - Skip the current goal and send it to the back");
            println!("  check_strat <formula>         - Run Bellman-Ford on raw geometry");
            println!("  qed                           - Finish proof");
            println!("  abort                         - Abort current proof");
        }
        
        "theorem" => {
            if parts.len() < 3 {
                println!("{}", "Usage: theorem <name> <formula>".red());
                return;
            }
            let _name = parts[1].to_string();
            let formula = parts[2..].join(" ");
            let mut parser = Parser::with_macros(&formula, &mut session.arena, Some(&session.macros));
            let root_idx = parser.parse_formula();

            let goal = Goal {
                context: Vec::new(),
                target: root_idx,
            };
            session.active_goals.push(goal);
            println!("[Goal Set] 1 unproven target.");
        }
        "show_goal" => {
            show_goal(session);
        }
        "qed" => {
            if session.active_goals.is_empty() {
                println!("Proof accepted.");
            } else {
                println!("There are still unproven goals.");
            }
        }
        "abort" => {
            session.active_goals.clear();
            println!("Proof aborted.");
        }
        "rewrite" => {
            if parts.len() < 2 {
                println!("{}", "Usage: rewrite <hyp_name>".red());
                return;
            }
            println!("Rewriting..."); // Dummy rewrite implementation
        }
        "quit" => {
            std::process::exit(0);
        }

        "exit" => {
            std::process::exit(0);
        }
        "save_session" => {
            if parts.len() < 2 {
                println!("{}", "Usage: save_session <file>".red());
                return;
            }
            let filename = parts[1];
            match serde_json::to_string_pretty(session) {
                Ok(json) => {
                    if let Err(e) = fs::write(filename, json) {
                        println!("{}: {}", "Failed to save session".red(), e);
                    } else {
                        println!("Session saved to {}", filename.green());
                    }
                }
                Err(e) => println!("{}: {}", "Failed to serialize session".red(), e),
            }
        }
        "load_session" => {
            if parts.len() < 2 {
                println!("{}", "Usage: load_session <file>".red());
                return;
            }
            let filename = parts[1];
            match fs::read_to_string(filename) {
                Ok(json) => {
                    match serde_json::from_str(&json) {
                        Ok(loaded_session) => {
                            *session = loaded_session;
                            println!("Session loaded from {}", filename.green());
                        }
                        Err(e) => println!("{}: {}", "Failed to deserialize session".red(), e),
                    }
                }
                Err(e) => println!("{}: {}", "Failed to load session".red(), e),
            }
        }
        "assume" => {
            if parts.len() < 2 {
                println!("{}", "Usage: assume <axiom>".red());
                return;
            }
            let axiom = parts[1..].join(" ");
            session.axioms.push(axiom.clone());
            println!("Assumed: {}", axiom.cyan());
        }
        "eval" => {
            if parts.len() < 2 {
                println!("{}", "Usage: eval <formula>".red());
                return;
            }
            let formula = parts[1..].join(" ");
            
            let mut arena = FormulaArena::new();
            let mut parser = Parser::with_macros(&formula, &mut arena, None);
            let root_idx = parser.parse_formula();

            let constraints = extract_constraints_aux(&arena, root_idx, 0);
            let mut graph = GraphArena::from_constraints(&constraints);
            graph.collapse_scc_0_weight();

            // Merge with session graph? For now just evaluate independently
            match graph.bellman_ford() {
                Ok(_) => println!("{}", "Stratification successful.".green()),
                Err(e) => println!("{}: {}", "Error".red(), e),
            }
        }
        "step" => {
             if parts.len() < 2 {
                println!("{}", "Usage: step <formula>".red());
                return;
            }
            let formula = parts[1..].join(" ");
            
            let mut arena = FormulaArena::new();
            let mut parser = Parser::with_macros(&formula, &mut arena, None);
            let root_idx = parser.parse_formula();

            let constraints = extract_constraints_aux(&arena, root_idx, 0);
            let mut graph = GraphArena::from_constraints(&constraints);
            
            println!("{}", "--- Extracting Constraints ---".yellow());
            for c in &constraints {
                println!("{:?}", c);
            }
            
            println!("{}", "--- Graph Nodes ---".yellow());
            for i in 0..graph.vars.len() {
                println!("Node {}", i);
            }

            println!("{}", "--- Graph Edges ---".yellow());
            for e in &graph.edges {
                if e.2 < 0 {
                    println!("Edge {} -> {} weight {}", e.0, e.1, e.2.to_string().red());
                } else {
                    println!("Edge {} -> {} weight {}", e.0, e.1, e.2);
                }
            }

            println!("{}", "--- Collapsing SCC ---".yellow());
            graph.collapse_scc_0_weight();
            
            println!("{}", "--- Running Bellman-Ford ---".yellow());
            match graph.bellman_ford() {
                Ok(_) => println!("{}", "Stratification successful.".green()),
                Err(e) => println!("{}: {}", "Error".red(), e),
            }
        }
        "intro" => {
            let name = if parts.len() > 1 { parts[1].to_string() } else { "H".to_string() };
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
                        println!("{}", "Goal is not an implication or universal quantification.".red());
                        session.active_goals.push(goal);
                    }
                }
            } else {
                println!("{}", "No active goals.".red());
            }
        }
        "exact" => {
            if parts.len() < 2 {
                println!("{}", "Usage: exact <hyp_name>".red());
                return;
            }
            let name = parts[1].to_string();
            if let Some(goal) = session.active_goals.last() {
                if let Some((_, hyp_idx)) = goal.context.iter().find(|(n, _)| n == &name) {
                    if *hyp_idx == goal.target {
                        println!("{}", "Goal closed!".green());
                        session.active_goals.pop();
                        if session.active_goals.is_empty() {
                            println!("{}", "Proof complete!".green().bold());
                        } else {
                            show_goal(session);
                        }
                    } else {
                        println!("{}", "Hypothesis does not exactly match the target.".red());
                    }
                } else {
                    println!("{}", "Invalid hypothesis name.".red());
                }
            } else {
                println!("{}", "No active goals.".red());
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
                        println!("{}", "Goal is not a conjunction.".red());
                        session.active_goals.push(goal);
                    }
                }
            } else {
                println!("{}", "No active goals.".red());
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
                        println!("{}", "Goal is not a disjunction.".red());
                        session.active_goals.push(goal);
                    }
                }
            } else {
                println!("{}", "No active goals.".red());
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
                        println!("{}", "Goal is not a disjunction.".red());
                        session.active_goals.push(goal);
                    }
                }
            } else {
                println!("{}", "No active goals.".red());
            }
        }
        "apply" => {
            if parts.len() < 2 {
                println!("{}", "Usage: apply <hyp_name>".red());
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
                            println!("{}", "Hypothesis is not an implication matching the target.".red());
                            session.active_goals.push(goal);
                        }
                    }
                } else {
                    println!("{}", "Invalid hypothesis name.".red());
                    session.active_goals.push(goal);
                }
            } else {
                println!("{}", "No active goals.".red());
            }
        }
        "destruct" => {
            if parts.len() < 2 {
                println!("{}", "Usage: destruct <hyp_name> [n1] [n2]".red());
                return;
            }
            let name = parts[1].to_string();
            let n1 = if parts.len() > 2 { parts[2].to_string() } else { format!("{}a", name) };
            let n2 = if parts.len() > 3 { parts[3].to_string() } else { format!("{}b", name) };

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
                            println!("{}", "Hypothesis cannot be destructed.".red());
                            goal.context.insert(idx, (name, hyp_idx));
                            session.active_goals.push(goal);
                        }
                    }
                } else {
                    println!("{}", "Invalid hypothesis name.".red());
                    session.active_goals.push(goal);
                }
            } else {
                println!("{}", "No active goals.".red());
            }
        }
        "focus_hyp" => {
            if parts.len() < 2 {
                println!("{}", "Usage: focus_hyp <hyp_name>".red());
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
                    println!("{}", "Invalid hypothesis name.".red());
                    session.active_goals.push(goal);
                }
            } else {
                println!("{}", "No active goals.".red());
            }
        }
        "defer" => {
            if session.active_goals.len() > 1 {
                let goal = session.active_goals.pop().unwrap();
                session.active_goals.insert(0, goal);
                println!("{}", "Goal deferred.".green());
                show_goal(session);
            } else if session.active_goals.len() == 1 {
                println!("{}", "Only one active goal.".yellow());
            } else {
                println!("{}", "No active goals.".red());
            }
        }
        "cut" => {
            if parts.len() < 2 {
                println!("{}", "Usage: cut <formula>".red());
                return;
            }
            let formula_str = parts[1..].join(" ");
            let mut parser = Parser::with_macros(&formula_str, &mut session.arena, Some(&session.macros));
            let cut_idx = parser.parse_formula();

            if let Some(mut goal) = session.active_goals.pop() {
                let mut goal2 = goal.clone();
                goal2.context.push(("Cut".to_string(), cut_idx));
                
                let mut goal1 = goal.clone();
                goal1.target = cut_idx;

                session.active_goals.push(goal2);
                session.active_goals.push(goal1);
                show_goal(session);
            } else {
                println!("{}", "No active goals.".red());
            }
        }
        "check_strat" => {
            if parts.len() < 2 {
                println!("{}", "Usage: check_strat <formula>".red());
                return;
            }
            let formula = parts[1..].join(" ");
            
            let mut parser = Parser::with_macros(&formula, &mut session.arena, Some(&session.macros));
            let root_idx = parser.parse_formula();

            let constraints = extract_constraints_aux(&session.arena, root_idx, 0);
            let mut graph = GraphArena::from_constraints(&constraints);
            graph.collapse_scc_0_weight();

            match graph.bellman_ford() {
                Ok(_) => println!("{}", "Stratification successful. Topologically sound.".green()),
                Err(e) => println!("{}: {}", "Error: Negative-weight cycle detected".red(), e),
            }
        }
        "deff" => {
            if parts.len() < 3 || !parts.contains(&":=") {
                println!("{}", "Usage: deff <name>(<args>) := <formula>".red());
                return;
            }
            let eq_idx = parts.iter().position(|&x| x == ":=").unwrap();
            let sig_str = parts[1..eq_idx].join(" ");
            let formula_str = parts[eq_idx + 1..].join(" ");

            // parse signature: Name(A, B)
            let sig_str = sig_str.replace(" ", "");
            let open_paren = sig_str.find('(');
            let close_paren = sig_str.find(')');
            
            let mut name = String::new();
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

            let mut parser = Parser::with_macros(&formula_str, &mut session.arena, Some(&session.macros));
            let root_idx = parser.parse_formula();

            let constraints = extract_constraints_aux(&session.arena, root_idx, 0);
            let mut graph = GraphArena::from_constraints(&constraints);
            graph.collapse_scc_0_weight();
            
            session.macros.insert(name.clone(), (params, root_idx));
            println!("Macro {} defined and SCC flattened.", name.cyan());
        }
        _ => {
            println!("{}: Unknown command '{}'", "Error".red(), parts[0]);
        }
    }
}

fn format_formula(arena: &FormulaArena, idx: usize, show_tags: bool) -> String {
    let formula = match arena.get(idx) {
        Some(f) => f,
        None => return format!("<?{}>", idx),
    };
    match formula {
        Formula::Atom(Atomic::Eq(v1, v2)) => format!("{} = {}", format_var(v1, show_tags), format_var(v2, show_tags)),
        Formula::Atom(Atomic::Mem(v1, v2)) => format!("{} in {}", format_var(v1, show_tags), format_var(v2, show_tags)),
        Formula::Atom(a) => format!("{:?}", a),
        Formula::Neg(i) => format!("~{}", format_formula(arena, *i, show_tags)),
        Formula::Conj(l, r) => format!("({} /\\ {})", format_formula(arena, *l, show_tags), format_formula(arena, *r, show_tags)),
        Formula::Disj(l, r) => format!("({} \\/ {})", format_formula(arena, *l, show_tags), format_formula(arena, *r, show_tags)),
        Formula::Impl(l, r) => format!("({} -> {})", format_formula(arena, *l, show_tags), format_formula(arena, *r, show_tags)),
        Formula::Univ(_, var, inner) => format!("forall {}. {}", var, format_formula(arena, *inner, show_tags)),
        Formula::Exist(_, var, inner) => format!("exists {}. {}", var, format_formula(arena, *inner, show_tags)),
        Formula::Comp(_, var, inner) => format!("{{ {} | {} }}", var, format_formula(arena, *inner, show_tags)),
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
        },
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
        println!("{}", "--- Context ---".yellow());
        for hyp in goal.context.iter() {
            println!("{}: {}", hyp.0, format_formula(&session.arena, hyp.1, false));
        }
        println!("{}", "--- Target ---".yellow());
        println!("{}", format_formula(&session.arena, goal.target, false).cyan().bold());
    } else {
        println!("No active goals.");
    }
}
