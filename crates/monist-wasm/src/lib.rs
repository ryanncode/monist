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
        let mut graph = GraphArena::from_constraints(&constraints);

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

#[wasm_bindgen]
pub struct ReplWasmSession {
    inner: monist_seq::itp::ReplSession,
}

#[wasm_bindgen]
impl ReplWasmSession {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: monist_seq::itp::ReplSession::new(),
        }
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

    pub fn execute_tactic(&mut self, cmd: String, args: Vec<String>) -> Result<(), JsValue> {
        let res = match cmd.as_str() {
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
            _ => Err("Unknown tactic.".to_string()),
        };

        res.map_err(|e| JsValue::from_str(&e))
    }

    pub fn get_state_json(&self) -> Result<JsValue, JsValue> {
        match &self.inner.active_state {
            Some(state) => {
                let val = serde_wasm_bindgen::to_value(state)
                    .map_err(|e| JsValue::from_str(&e.to_string()))?;
                Ok(val)
            }
            None => Ok(JsValue::NULL)
        }
    }
}
