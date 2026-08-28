use wasm_bindgen::prelude::*;
use monist_parser::parser::Parser;
use monist_core::ast::FormulaArena;
use monist_core::graph::GraphArena;
use monist_core::eval::{ExecutionLimits, evaluate_clause, EvalResult};
use monist_core::smt::export_smt_lib;
use monist_core::budget::ResourceBudget;

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct EvaluationResult {
    pub is_stratified: bool,
    pub max_k_iterations: usize,
    pub mcm: f64,
    smt_witness: String,
}

#[wasm_bindgen]
impl EvaluationResult {
    #[wasm_bindgen(getter)]
    pub fn smt_witness(&self) -> String {
        self.smt_witness.clone()
    }
}

#[wasm_bindgen]
pub fn evaluate_formula(input: &str) -> Result<EvaluationResult, JsValue> {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let budget = ResourceBudget::default();
        let mut arena = FormulaArena::new();
        let mut parser = Parser::new(input, &mut arena, budget);
        let formula_idx = parser.parse_formula();

        let mut edge_count = 0;
        let constraints = monist_core::graph::extract_constraints_aux(&arena, formula_idx, 0, false, &budget, &mut edge_count);
        let graph = GraphArena::from_constraints(&constraints);

        let limits = ExecutionLimits::compute_for_graph(&graph)
            .ok_or_else(|| JsValue::from_str("Numeric Overflow in Execution Limits DP"))?;

        let is_stratified = limits.mcm >= 0.0;
        
        // Evaluate to get success depths or negative cycle
        let edges = graph.edges.iter().map(|(u, v, w, in_comp)| {
            monist_core::graph::Edge {
                source: graph.vars[*u].clone(),
                target: graph.vars[*v].clone(),
                weight: *w,
                in_comp: *in_comp,
            }
        }).collect::<Vec<_>>();
        
        let eval_res = evaluate_clause(&edges);
        let (collision_trace, success_depths) = match eval_res {
            EvalResult::NegativeCycle => (Some("Negative Cycle Detected"), None),
            EvalResult::NumericOverflow => panic!("Numeric Overflow during evaluation"),
            EvalResult::Success(depths) => {
                let just_depths: Vec<i32> = depths.iter().map(|(_, d)| *d).collect();
                (None, Some(just_depths))
            }
        };
        
        let depths_ref = success_depths.as_deref();

        let smt_witness = export_smt_lib(&graph, input, collision_trace, &[], depths_ref);

        Ok::<_, JsValue>(EvaluationResult {
            is_stratified,
            max_k_iterations: limits.max_k_iterations,
            mcm: limits.mcm,
            smt_witness,
        })
    }));

    match result {
        Ok(res) => res,
        Err(err) => {
            let msg = if let Some(s) = err.downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = err.downcast_ref::<String>() {
                s.clone()
            } else {
                "Unknown panic in parser.".to_string()
            };
            Err(JsValue::from_str(&format!("Syntax/Parse Error: {}", msg)))
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct FormattedHypothesis {
    pub name: String,
    pub formula: String,
    pub raw_node: usize,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct FormattedGoal {
    pub ctx: Vec<FormattedHypothesis>,
    pub target: String,
    pub raw_target_node: usize,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct FormattedProofState {
    pub goals: Vec<FormattedGoal>,
    pub is_stratified: bool,
    pub mcm: f64,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ReplResponse {
    pub success: bool,
    pub output_lines: Vec<String>,
    pub state: Option<FormattedProofState>,
    pub error: Option<String>,
}

#[wasm_bindgen]
pub struct ReplWasmSession {
    inner: monist_seq::itp::ReplSession,
}

impl ReplWasmSession {
    fn compute_goal_stratification(&self, goal: &monist_seq::itp::Goal) -> (bool, f64) {
        let budget = ResourceBudget::default();
        let mut edge_count = 0;
        let mut constraints = Vec::new();
        for (_, hyp_idx) in &goal.ctx {
            constraints.extend(monist_core::graph::extract_constraints_aux(
                &self.inner.arena, *hyp_idx, 0, false, &budget, &mut edge_count
            ));
        }
        constraints.extend(monist_core::graph::extract_constraints_aux(
            &self.inner.arena, goal.target, 0, false, &budget, &mut edge_count
        ));
        let graph = GraphArena::from_constraints(&constraints);
        if let Some(limits) = ExecutionLimits::compute_for_graph(&graph) {
            (limits.mcm >= 0.0, limits.mcm)
        } else {
            (false, -1.0)
        }
    }

    fn get_formatted_state(&self) -> Option<FormattedProofState> {
        let state = self.inner.active_state.as_ref()?;
        let (is_stratified, mcm) = if let Some(first_goal) = state.goals.first() {
            self.compute_goal_stratification(first_goal)
        } else {
            (true, 0.0)
        };

        let goals = state.goals.iter().map(|g| {
            let ctx = g.ctx.iter().map(|(name, idx)| {
                FormattedHypothesis {
                    name: name.clone(),
                    formula: self.inner.format_formula(*idx),
                    raw_node: *idx,
                }
            }).collect();
            let target = self.inner.format_formula(g.target);
            FormattedGoal {
                ctx,
                target,
                raw_target_node: g.target,
            }
        }).collect();

        Some(FormattedProofState {
            goals,
            is_stratified,
            mcm,
        })
    }
}

#[wasm_bindgen]
impl ReplWasmSession {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: monist_seq::itp::ReplSession::new(),
        }
    }

    pub fn process_repl_line(&mut self, input: &str) -> Result<JsValue, JsValue> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            let resp = ReplResponse {
                success: true,
                output_lines: vec![],
                state: self.get_formatted_state(),
                error: None,
            };
            return serde_wasm_bindgen::to_value(&resp).map_err(|e| JsValue::from_str(&e.to_string()));
        }

        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        let cmd = parts[0];
        let mut output_lines = Vec::new();
        let mut error_msg = None;
        let mut success = true;

        match cmd {
            "theorem" => {
                if parts.len() < 3 {
                    error_msg = Some("Usage: theorem <name> <formula>".to_string());
                    success = false;
                } else {
                    let name = parts[1].to_string();
                    let target_str = parts[2..].join(" ");
                    let budget = ResourceBudget::default();
                    let mut parser = Parser::with_macros(&target_str, &mut self.inner.arena, Some(&self.inner.macros), budget);
                    let target_idx = parser.parse_formula();
                    self.inner.start_proof(name.clone(), target_idx);
                    output_lines.push(format!("Starting proof of {}", name));
                }
            }
            "assume" => {
                if parts.len() < 3 {
                    error_msg = Some("Usage: assume <name> <formula>".to_string());
                    success = false;
                } else {
                    let name = parts[1].to_string();
                    let formula_str = parts[2..].join(" ");
                    let budget = ResourceBudget::default();
                    let mut parser = Parser::with_macros(&formula_str, &mut self.inner.arena, Some(&self.inner.macros), budget);
                    let root_idx = parser.parse_formula();
                    self.inner.theorems.push((name.clone(), root_idx));
                    output_lines.push(format!("Axiom {} added.", name));
                }
            }
            "deff" => {
                if parts.len() < 3 || !parts.contains(&":=") {
                    error_msg = Some("Usage: deff <name>(<args>) := <formula>".to_string());
                    success = false;
                } else {
                    let eq_idx = parts.iter().position(|&x| x == ":=").unwrap();
                    let raw_sig = parts[1..eq_idx].join(" ");
                    let formula_str = parts[eq_idx + 1..].join(" ");
                    let name;
                    let mut params = Vec::new();
                    if raw_sig.contains('(') && raw_sig.contains(')') {
                        let sig_str = raw_sig.replace(" ", "");
                        let op = sig_str.find('(').unwrap();
                        let cp = sig_str.find(')').unwrap();
                        name = sig_str[..op].to_string();
                        let params_str = &sig_str[op + 1..cp];
                        if !params_str.is_empty() {
                            params = params_str.split(',').map(|s| s.to_string()).collect();
                        }
                    } else {
                        let tokens: Vec<&str> = parts[1..eq_idx].iter().cloned().filter(|s| !s.is_empty()).collect();
                        if tokens.is_empty() {
                            name = "macro".to_string();
                        } else {
                            name = tokens[0].to_string();
                            params = tokens[1..].iter().map(|s| s.to_string()).collect();
                        }
                    }
                    match self.inner.define_macro(name.clone(), params, &formula_str) {
                        Ok(()) => output_lines.push(format!("Macro {} defined and SCC flattened.", name)),
                        Err(e) => { error_msg = Some(e); success = false; }
                    }
                }
            }
            "qed" => {
                if let Some(state) = &self.inner.active_state {
                    if state.goals.is_empty() {
                        output_lines.push("Proof accepted.".to_string());
                        self.inner.active_state = None;
                    } else {
                        error_msg = Some(format!("Cannot finish proof: {} goal(s) remaining.", state.goals.len()));
                        success = false;
                    }
                } else {
                    error_msg = Some("No active proof to finish.".to_string());
                    success = false;
                }
            }
            "abort" => {
                self.inner.active_state = None;
                output_lines.push("Proof aborted.".to_string());
            }
            "show_goal" => {
                if let Some(state) = &self.inner.active_state {
                    if let Some(goal) = state.goals.first() {
                        output_lines.push("--- Context ---".to_string());
                        for (name, idx) in &goal.ctx {
                            output_lines.push(format!("  {} : {}", name, self.inner.format_formula(*idx)));
                        }
                        output_lines.push("----------------------".to_string());
                        output_lines.push(format!("  {}", self.inner.format_formula(goal.target)));
                    } else {
                        output_lines.push("No active goals. Proof complete! Type 'qed' to finish.".to_string());
                    }
                } else {
                    output_lines.push("No active proof.".to_string());
                }
            }
            "eval" | "check_strat" => {
                if parts.len() < 2 {
                    error_msg = Some(format!("Usage: {} <formula>", cmd));
                    success = false;
                } else {
                    let formula_str = parts[1..].join(" ");
                    match evaluate_formula(&formula_str) {
                        Ok(res) => {
                            if res.is_stratified {
                                output_lines.push(format!("Stratification successful. MCM = {:.2}, Max K-Iterations = {}", res.mcm, res.max_k_iterations));
                            } else {
                                output_lines.push(format!("Extensionality Collision! Negative-weight cycle detected (MCM = {:.2})", res.mcm));
                            }
                        }
                        Err(e) => {
                            error_msg = Some(format!("{:?}", e));
                            success = false;
                        }
                    }
                }
            }
            // Tactic execution
            _ => {
                let args: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();
                let res = match cmd {
                    "intro" => {
                        let name = args.get(0).cloned().unwrap_or_else(|| "H".to_string());
                        self.inner.tactic_intro(name)
                    }
                    "exact" => {
                        let name = args.get(0).cloned().unwrap_or_default();
                        self.inner.tactic_exact(&name)
                    }
                    "apply" => {
                        let name = args.get(0).cloned().unwrap_or_default();
                        self.inner.tactic_apply(&name)
                    }
                    "split" => self.inner.tactic_split(),
                    "left" => self.inner.tactic_left(),
                    "right" => self.inner.tactic_right(),
                    "destruct" => {
                        let name = args.get(0).cloned().unwrap_or_default();
                        let n1 = args.get(1).cloned().unwrap_or_default();
                        let n2 = args.get(2).cloned().unwrap_or_default();
                        self.inner.tactic_destruct(&name, n1, n2)
                    }
                    "cut" => {
                        let formula_str = args.join(" ");
                        self.inner.tactic_cut(&formula_str)
                    }
                    "stratify" => self.inner.tactic_stratify(),
                    "refl" => self.inner.tactic_refl(),
                    "have" => {
                        let name = args.get(0).cloned().unwrap_or_else(|| "H".to_string());
                        let formula_str = args.iter().skip(1).cloned().collect::<Vec<_>>().join(" ");
                        self.inner.tactic_have(&name, &formula_str)
                    }
                    "collapse_loop" => self.inner.tactic_collapse_loop(),
                    "schonfinkel" => self.inner.tactic_schonfinkel(),
                    "step" => self.inner.tactic_step(),
                    "simp" => self.inner.tactic_simp(),
                    "rw" | "rewrite" => {
                        let name = args.get(0).cloned().unwrap_or_default();
                        self.inner.tactic_rw(&name)
                    }
                    "focus_hyp" => {
                        let name = args.get(0).cloned().unwrap_or_default();
                        self.inner.tactic_focus_hyp(&name)
                    }
                    "defer" => self.inner.tactic_defer(),
                    "elevate" => {
                        let name = args.get(0).cloned().unwrap_or_default();
                        self.inner.tactic_elevate(&name)
                    }
                    "sc_cut" => {
                        let name = args.get(0).cloned().unwrap_or_else(|| "x".to_string());
                        self.inner.tactic_sc_cut(&name)
                    }
                    _ => Err(format!("Unknown command or tactic '{}'", cmd)),
                };

                match res {
                    Ok(()) => {
                        if let Some(state) = &self.inner.active_state {
                            if state.goals.is_empty() {
                                output_lines.push("No more goals. Proof complete! Type 'qed' to finish.".to_string());
                            }
                        }
                    }
                    Err(e) => {
                        error_msg = Some(e);
                        success = false;
                    }
                }
            }
        }

        let resp = ReplResponse {
            success,
            output_lines,
            state: self.get_formatted_state(),
            error: error_msg,
        };

        serde_wasm_bindgen::to_value(&resp).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    pub fn start_proof(&mut self, name: String, target_str: String) -> Result<(), JsValue> {
        let budget = ResourceBudget::default();
        let mut parser = Parser::with_macros(&target_str, &mut self.inner.arena, Some(&self.inner.macros), budget);
        let target_idx = parser.parse_formula();
        self.inner.start_proof(name, target_idx);
        Ok(())
    }

    pub fn define_macro(&mut self, name: String, params: Vec<String>, formula_str: String) -> Result<(), JsValue> {
        self.inner.define_macro(name, params, &formula_str).map_err(|e| JsValue::from_str(&e))
    }

    pub fn get_state_json(&self) -> Result<JsValue, JsValue> {
        let formatted = self.get_formatted_state();
        serde_wasm_bindgen::to_value(&formatted).map_err(|e| JsValue::from_str(&e.to_string()))
    }
}
