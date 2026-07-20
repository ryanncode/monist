use crate::ast::Var;
use crate::graph::{GraphArena, ScopedVar};

pub fn export_smt_lib(
    arena: &GraphArena,
    formula_name: &str,
    collision_trace: Option<&str>,
    sc_actions: &[String],
    success_depths: Option<&[i32]>,
) -> String {
    let mut out = String::new();

    out.push_str("; === BEGIN STRATIFICATION WITNESS ===\n");
    out.push_str(&format!("(set-info :formula-name \"{}\")\n", escape_smt_string(formula_name)));

    if let Some(trace) = collision_trace {
        out.push_str(&format!("(set-info :extensionality-collision-trace \"{}\")\n", escape_smt_string(trace)));
    }

    if !sc_actions.is_empty() {
        let actions_str = sc_actions.join("\n");
        out.push_str(&format!("(set-info :sc-daemon-actions \"{}\")\n", escape_smt_string(&actions_str)));
    }

    if let Some(depths) = success_depths {
        let mut depth_str = String::new();
        for (i, d) in depths.iter().enumerate() {
            let var_name = format!("v{}", i);
            depth_str.push_str(&format!("{} -> {}\n", var_name, d));
        }
        out.push_str(&format!("(set-info :stratification-success-depths \"{}\")\n", escape_smt_string(depth_str.trim_end())));
    }

    out.push_str("(set-logic QF_LIA)\n\n");

    // Declare variables
    for (i, var) in arena.vars.iter().enumerate() {
        let name = format!("v{}", i);
        let ScopedVar(v, depth) = var;
        let orig_name = match v {
            Var::Free(s) => s.clone(),
            Var::Bound(idx) => format!("b{}", idx),
        };
        out.push_str(&format!("; original variable: {} depth: {}\n", escape_smt_string(&orig_name), depth));
        out.push_str(&format!("(declare-fun {} () Int)\n", name));
    }
    out.push_str("\n");

    // Assert constraints
    // Edge (u, v, w) means d[v] <= d[u] + w
    for &(u, v, w, _) in &arena.edges {
        let u_name = format!("v{}", u);
        let v_name = format!("v{}", v);
        out.push_str(&format!("(assert (<= (- {} {}) {}))\n", v_name, u_name, w));
    }

    out.push_str("\n(check-sat)\n");
    out.push_str("(get-model)\n");
    out.push_str("; === END STRATIFICATION WITNESS ===\n");

    out
}

fn escape_smt_string(s: &str) -> String {
    s.replace("\"", "\"\"")
}
