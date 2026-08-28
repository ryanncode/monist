pub mod ast;
pub mod eval;
pub mod graph;
pub mod smt;
pub mod budget;

#[cfg(test)]
mod tests {
    use super::*;
    use ast::{Atomic, Formula, FormulaArena, Var};
    use graph::{GraphArena, ScopedVar};
    use budget::ResourceBudget;

    #[test]
    fn test_tarjan_scc_0_weight() {
        let mut graph = GraphArena::new();
        let v0 = ScopedVar(Var::Free("x".to_string()), 0);
        let v1 = ScopedVar(Var::Free("y".to_string()), 0);
        let v2 = ScopedVar(Var::Free("z".to_string()), 0);

        let i0 = graph.add_var(v0);
        let i1 = graph.add_var(v1);
        let i2 = graph.add_var(v2);

        graph.edges.push((i0, i1, 0, false));
        graph.edges.push((i1, i0, 0, false));
        graph.edges.push((i0, i2, 1, false));

        let sccs = graph.tarjan_scc();
        let scc_0_1 = sccs.iter().find(|scc| scc.contains(&i0) && scc.contains(&i1));
        assert!(scc_0_1.is_some(), "Expected i0 and i1 in the same SCC");

        let (c_vars, _, reps) = graph.contract_graph(&sccs);
        assert_eq!(reps[i0], reps[i1], "i0 and i1 must share representatives");
        assert_eq!(c_vars.len(), 2, "Expected 2 contracted vertices");
    }

    #[test]
    fn test_dnf_disjunction_branching() {
        let mut arena = FormulaArena::new();
        let x_in_x = arena.add(Formula::Atom(Atomic::Mem(Var::Free("x".to_string()), Var::Free("x".to_string()))));
        let x_in_y = arena.add(Formula::Atom(Atomic::Mem(Var::Free("x".to_string()), Var::Free("y".to_string()))));
        let disj = arena.add(Formula::Disj(x_in_x, x_in_y));

        let res = GraphArena::evaluate_dnf_formula(&arena, disj, &ResourceBudget::default());
        assert!(res.is_ok(), "Disjunction with one stratifiable branch must succeed");
    }

    #[test]
    fn test_extensionality_collision_negative_cycle() {
        let mut arena = FormulaArena::new();
        let x_in_x = arena.add(Formula::Atom(Atomic::Mem(Var::Free("x".to_string()), Var::Free("x".to_string()))));
        let x_notin_x = arena.add(Formula::Neg(x_in_x));
        let comp = arena.add(Formula::Comp(0, "x".to_string(), x_notin_x));

        let res = GraphArena::evaluate_dnf_formula(&arena, comp, &ResourceBudget::default());
        assert!(res.is_err(), "Russell comprehension must fail with Extensionality Collision");
    }

    #[test]
    fn test_smt_export_validity() {
        let mut graph = GraphArena::new();
        let v0 = ScopedVar(Var::Free("a".to_string()), 0);
        let v1 = ScopedVar(Var::Free("b".to_string()), 0);
        let i0 = graph.add_var(v0);
        let i1 = graph.add_var(v1);
        graph.edges.push((i0, i1, 1, false));

        let smt = smt::export_smt_lib(&graph, "test_formula", None, &[], Some(&[0, 1]));
        assert!(smt.contains("(set-logic QF_LIA)"));
        assert!(smt.contains("(declare-fun v0 () Int)"));
        assert!(smt.contains("(declare-fun v1 () Int)"));
        assert!(smt.contains("(check-sat)"));
    }
}
