use clap::{Parser as ClapParser, Subcommand};
use colored::*;
use monist_core::ast::FormulaArena;
use monist_core::graph::{GraphArena, extract_constraints_aux};
use monist_core::smt::export_smt_lib;
use monist_parser::parser::Parser;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result as RlResult};
use serde::{Deserialize, Serialize};
use std::fs;

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
}

impl Default for Session {
    fn default() -> Self {
        Self {
            graph: GraphArena::new(),
            axioms: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Session {
    graph: GraphArena,
    axioms: Vec<String>,
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
        _ => {
            println!("{}: Unknown command '{}'", "Error".red(), parts[0]);
        }
    }
}
