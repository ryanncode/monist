use crate::graph::{GraphArena, ScopedVar};
use crate::ast::Var;

pub fn export_smt_lib(arena: &GraphArena, formula_name: &str) -> String {
    let mut out = String::new();
    out.push_str(&format!("; StratificationWitness for {}\n", formula_name));
    out.push_str("(set-logic QF_LIA)\n\n");
    
    // Declare variables
    for var in &arena.vars {
        let name = var_to_smt_name(var);
        out.push_str(&format!("(declare-fun {} () Int)\n", name));
    }
    out.push_str("\n");
    
    // Assert constraints
    // Edge (u, v, w) means d[v] <= d[u] + w
    for &(u, v, w) in &arena.edges {
        let u_name = var_to_smt_name(&arena.vars[u]);
        let v_name = var_to_smt_name(&arena.vars[v]);
        out.push_str(&format!("(assert (<= (- {} {}) {}))\n", v_name, u_name, w));
    }
    
    out.push_str("\n(check-sat)\n");
    out.push_str("(get-model)\n");
    
    out
}

fn var_to_smt_name(var: &ScopedVar) -> String {
    let ScopedVar(v, depth) = var;
    let base_name = match v {
        Var::Free(s) => s.clone(),
        Var::Bound(idx) => format!("b{}", idx),
    };
    format!("{}_d{}", base_name, depth)
}
