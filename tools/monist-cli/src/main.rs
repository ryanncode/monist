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
    context: Vec<usize>,
    target: usize,
}

#[derive(Serialize, Deserialize)]
struct Session {
    graph: GraphArena,
    axioms: Vec<String>,
    arena: FormulaArena,
    active_goals: Vec<Goal>,
}

impl Default for Session {
    fn default() -> Self {
        Self {
            graph: GraphArena::new(),
            axioms: Vec::new(),
            arena: FormulaArena::new(),
            active_goals: Vec::new(),
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
            let mut parser = Parser::new(formula, &mut arena);
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
            let mut parser = Parser::new(formula, &mut arena);
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
            let mut parser = Parser::new(formula, &mut arena);
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
            println!("  help                    - Show this help message");
            println!("  exit                    - Exit the REPL");
            println!("  save_session <file>     - Save current session to a JSON file");
            println!("  load_session <file>     - Load a session from a JSON file");
            println!("  assume <axiom>          - Add an axiom");
            println!("  eval <formula>          - Evaluate a formula");
            println!("  step <formula>          - Step-by-step diagnostic evaluation");
            println!("  goal <formula>          - Set a new goal to prove");
            println!("  show_goal               - Show the current goal state");
            println!("  intro [var]             - Introduce a hypothesis or variable");
            println!("  exact <hyp_idx>         - Close goal if it matches hypothesis exactly");
            println!("  split                   - Split a conjunction goal into two");
            println!("  left                    - Prove left side of a disjunction");
            println!("  right                   - Prove right side of a disjunction");
            println!("  apply <thm_idx>         - Apply a theorem/hypothesis (backward reasoning)");
            println!("  destruct <hyp_idx>      - Break down a hypothesis (e.g. conjunction)");
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
            let mut parser = Parser::new(&formula, &mut arena);
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
            let mut parser = Parser::new(&formula, &mut arena);
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
            if let Some(mut goal) = session.active_goals.pop() {
                let target = session.arena.get(goal.target).cloned();
                match target {
                    Some(Formula::Impl(l, r)) => {
                        goal.context.push(l);
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
                println!("{}", "Usage: exact <hyp_idx>".red());
                return;
            }
            if let Ok(idx) = parts[1].parse::<usize>() {
                if let Some(goal) = session.active_goals.last() {
                    if let Some(&hyp) = goal.context.get(idx) {
                        if hyp == goal.target {
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
                        println!("{}", "Invalid hypothesis index.".red());
                    }
                } else {
                    println!("{}", "No active goals.".red());
                }
            } else {
                println!("{}", "Invalid index.".red());
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
            // Apply backward reasoning: A -> B against target B changes target to A.
            // Wait, we need to find an implication in the context that matches the target.
            // Usage: apply <hyp_idx>
            if parts.len() < 2 {
                println!("{}", "Usage: apply <hyp_idx>".red());
                return;
            }
            if let Ok(idx) = parts[1].parse::<usize>() {
                if let Some(mut goal) = session.active_goals.pop() {
                    if let Some(&hyp) = goal.context.get(idx) {
                        let hyp_f = session.arena.get(hyp).cloned();
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
                        println!("{}", "Invalid hypothesis index.".red());
                        session.active_goals.push(goal);
                    }
                } else {
                    println!("{}", "No active goals.".red());
                }
            } else {
                println!("{}", "Invalid index.".red());
            }
        }
        "destruct" => {
            // Break down a hypothesis. e.g., A /\ B -> adds A and B to context.
            // A \/ B -> splits into two goals, one with A, one with B.
            if parts.len() < 2 {
                println!("{}", "Usage: destruct <hyp_idx>".red());
                return;
            }
            if let Ok(idx) = parts[1].parse::<usize>() {
                if let Some(mut goal) = session.active_goals.pop() {
                    if idx < goal.context.len() {
                        let hyp = goal.context.remove(idx); // remove it
                        let hyp_f = session.arena.get(hyp).cloned();
                        match hyp_f {
                            Some(Formula::Conj(l, r)) => {
                                goal.context.push(l);
                                goal.context.push(r);
                                session.active_goals.push(goal);
                                show_goal(session);
                            }
                            Some(Formula::Disj(l, r)) => {
                                let mut goal2 = goal.clone();
                                goal.context.push(l);
                                goal2.context.push(r);
                                session.active_goals.push(goal2);
                                session.active_goals.push(goal);
                                show_goal(session);
                            }
                            Some(Formula::Exist(_, _, inner)) => {
                                goal.context.push(inner);
                                session.active_goals.push(goal);
                                show_goal(session);
                            }
                            _ => {
                                println!("{}", "Hypothesis cannot be destructed.".red());
                                goal.context.insert(idx, hyp); // put it back
                                session.active_goals.push(goal);
                            }
                        }
                    } else {
                        println!("{}", "Invalid hypothesis index.".red());
                        session.active_goals.push(goal);
                    }
                } else {
                    println!("{}", "No active goals.".red());
                }
            } else {
                println!("{}", "Invalid index.".red());
            }
        }
        _ => {
            println!("{}: Unknown command '{}'", "Error".red(), parts[0]);
        }
    }
}

fn format_formula(arena: &FormulaArena, idx: usize) -> String {
    let formula = match arena.get(idx) {
        Some(f) => f,
        None => return format!("<?{}>", idx),
    };
    match formula {
        Formula::Atom(Atomic::Eq(v1, v2)) => format!("{} = {}", format_var(v1), format_var(v2)),
        Formula::Atom(Atomic::Mem(v1, v2)) => format!("{} in {}", format_var(v1), format_var(v2)),
        Formula::Atom(a) => format!("{:?}", a),
        Formula::Neg(i) => format!("~{}", format_formula(arena, *i)),
        Formula::Conj(l, r) => format!("({} /\\ {})", format_formula(arena, *l), format_formula(arena, *r)),
        Formula::Disj(l, r) => format!("({} \\/ {})", format_formula(arena, *l), format_formula(arena, *r)),
        Formula::Impl(l, r) => format!("({} -> {})", format_formula(arena, *l), format_formula(arena, *r)),
        Formula::Univ(_, var, inner) => format!("forall {}. {}", var, format_formula(arena, *inner)),
        Formula::Exist(_, var, inner) => format!("exists {}. {}", var, format_formula(arena, *inner)),
        Formula::Comp(_, var, inner) => format!("{{ {} | {} }}", var, format_formula(arena, *inner)),
    }
}

fn format_var(v: &Var) -> String {
    match v {
        Var::Free(name) => name.clone(),
        Var::Bound(idx) => format!("^{}", idx),
    }
}
// I'll append the logic for the commands to process_repl_command using a diff/edit next.

fn show_goal(session: &Session) {
    if let Some(goal) = session.active_goals.last() {
        println!("{}", "--- Context ---".yellow());
        for (i, &hyp) in goal.context.iter().enumerate() {
            println!("H{}: {}", i, format_formula(&session.arena, hyp));
        }
        println!("{}", "--- Target ---".yellow());
        println!("{}", format_formula(&session.arena, goal.target).cyan().bold());
    } else {
        println!("No active goals.");
    }
}
