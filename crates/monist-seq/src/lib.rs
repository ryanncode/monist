pub mod telemetry;
pub mod itp;

#[cfg(test)]
mod tests {
    use super::*;
    use itp::ReplSession;
    use monist_core::ast::{Atomic, Formula, Var};

    #[test]
    fn test_itp_intro_and_exact() {
        let mut session = ReplSession::new();
        let p_atom = session.arena.add(Formula::Atom(Atomic::Eq(Var::Free("x".to_string()), Var::Free("y".to_string()))));
        let impl_form = session.arena.add(Formula::Impl(p_atom, p_atom));

        session.start_proof("identity".to_string(), impl_form);
        assert_eq!(session.active_state.as_ref().unwrap().goals.len(), 1);

        session.tactic_intro("H".to_string()).expect("intro should succeed on Impl");
        assert_eq!(session.active_state.as_ref().unwrap().goals[0].ctx.len(), 1);

        session.tactic_exact("H").expect("exact should close the goal");
        assert_eq!(session.active_state.as_ref().unwrap().goals.len(), 0);
    }

    #[test]
    fn test_itp_universal_intro() {
        let mut session = ReplSession::new();
        let x_eq_x = session.arena.add(Formula::Atom(Atomic::Eq(Var::Bound(0), Var::Bound(0))));
        let forall_form = session.arena.add(Formula::Univ(0, "x".to_string(), x_eq_x));

        session.start_proof("forall_refl".to_string(), forall_form);
        session.tactic_intro("a".to_string()).expect("intro on forall must succeed");

        let target_formula = session.arena.get(session.active_state.as_ref().unwrap().goals[0].target).unwrap();
        assert_eq!(
            *target_formula,
            Formula::Atom(Atomic::Eq(Var::Free("a".to_string()), Var::Free("a".to_string())))
        );

        session.tactic_refl().expect("refl should close reflexivity goal");
        assert_eq!(session.active_state.as_ref().unwrap().goals.len(), 0);
    }

    #[test]
    fn test_itp_split() {
        let mut session = ReplSession::new();
        let a = session.arena.add(Formula::Atom(Atomic::Eq(Var::Free("a".to_string()), Var::Free("a".to_string()))));
        let b = session.arena.add(Formula::Atom(Atomic::Eq(Var::Free("b".to_string()), Var::Free("b".to_string()))));
        let conj = session.arena.add(Formula::Conj(a, b));

        session.start_proof("split_test".to_string(), conj);
        session.tactic_split().expect("split must succeed on Conj");
        assert_eq!(session.active_state.as_ref().unwrap().goals.len(), 2);
    }

    #[test]
    fn test_itp_stratify_tactic() {
        let mut session = ReplSession::new();
        let x_in_y = session.arena.add(Formula::Atom(Atomic::Mem(Var::Free("x".to_string()), Var::Free("y".to_string()))));
        session.start_proof("strat_test".to_string(), x_in_y);
        session.tactic_stratify().expect("stratify should close stratifiable goal");
        assert_eq!(session.active_state.as_ref().unwrap().goals.len(), 0);
    }

    #[test]
    fn test_itp_rw_tactic() {
        let mut session = ReplSession::new();
        let eq_ab = session.arena.add(Formula::Atom(Atomic::Eq(Var::Free("a".to_string()), Var::Free("b".to_string()))));
        let eq_ac = session.arena.add(Formula::Atom(Atomic::Eq(Var::Free("a".to_string()), Var::Free("c".to_string()))));
        
        session.theorems.push(("H_eq".to_string(), eq_ab));
        session.start_proof("test_rw".to_string(), eq_ac);
        
        assert_eq!(session.format_formula(session.active_state.as_ref().unwrap().goals[0].target), "a = c");
        session.tactic_rw("H_eq").expect("rewrite should succeed");
        assert_eq!(session.format_formula(session.active_state.as_ref().unwrap().goals[0].target), "b = c");
    }

    #[test]
    fn test_itp_simp_tactic() {
        let mut session = ReplSession::new();
        let a = session.arena.add(Formula::Atom(Atomic::Eq(Var::Free("a".to_string()), Var::Free("a".to_string()))));
        let b = session.arena.add(Formula::Atom(Atomic::Eq(Var::Free("b".to_string()), Var::Free("b".to_string()))));
        let disj = session.arena.add(Formula::Disj(a, b));
        let neg_disj = session.arena.add(Formula::Neg(disj));

        session.start_proof("test_simp".to_string(), neg_disj);
        session.tactic_simp().expect("simp should push negations inward");

        let target_str = session.format_formula(session.active_state.as_ref().unwrap().goals[0].target);
        assert_eq!(target_str, "(¬a = a ∧ ¬b = b)");
    }

    #[test]
    fn test_itp_elevate_tactic() {
        let mut session = ReplSession::new();
        let eq_ab = session.arena.add(Formula::Atom(Atomic::Eq(Var::Free("x".to_string()), Var::Free("y".to_string()))));
        
        session.start_proof("test_elevate".to_string(), eq_ab);
        session.tactic_elevate("x").expect("elevate should apply T-functor shift");

        let target_str = session.format_formula(session.active_state.as_ref().unwrap().goals[0].target);
        assert_eq!(target_str, "x_iota = y_iota");
    }

    #[test]
    fn test_itp_collapse_loop_tactic() {
        let mut session = ReplSession::new();
        let eq_xy = session.arena.add(Formula::Atom(Atomic::Eq(Var::Free("x".to_string()), Var::Free("y".to_string()))));
        let eq_yx = session.arena.add(Formula::Atom(Atomic::Eq(Var::Free("y".to_string()), Var::Free("x".to_string()))));
        let loop_form = session.arena.add(Formula::Conj(eq_xy, eq_yx));

        session.start_proof("test_collapse".to_string(), loop_form);
        session.tactic_collapse_loop().expect("collapse_loop should succeed");
        
        let target_str = session.format_formula(session.active_state.as_ref().unwrap().goals[0].target);
        assert!(target_str.contains(" = "));
    }

    #[test]
    fn test_itp_sc_cut_tactic() {
        let mut session = ReplSession::new();
        let eq_xy = session.arena.add(Formula::Atom(Atomic::Eq(Var::Free("x".to_string()), Var::Free("y".to_string()))));
        
        session.start_proof("test_sc_cut".to_string(), eq_xy);
        session.tactic_sc_cut("S").expect("sc_cut should inject Cantorian bedrock");

        let ctx = &session.active_state.as_ref().unwrap().goals[0].ctx;
        assert!(ctx.iter().any(|(name, _)| name == "SC_BEDROCK_S"));
    }

    #[test]
    fn test_itp_destruct_and_apply_workflow() {
        let mut session = ReplSession::new();
        let p = session.arena.add(Formula::Atom(Atomic::Eq(Var::Free("p".to_string()), Var::Free("p".to_string()))));
        let q = session.arena.add(Formula::Atom(Atomic::Eq(Var::Free("q".to_string()), Var::Free("q".to_string()))));
        let p_imp_q = session.arena.add(Formula::Impl(p, q));
        let conj = session.arena.add(Formula::Conj(p_imp_q, p));
        let modus_ponens = session.arena.add(Formula::Impl(conj, q));

        session.start_proof("modus_ponens".to_string(), modus_ponens);
        session.tactic_intro("H".to_string()).expect("intro H");
        session.tactic_destruct("H", "H_imp".to_string(), "H_p".to_string()).expect("destruct H");
        session.tactic_apply("H_imp").expect("apply H_imp");
        session.tactic_exact("H_p").expect("exact H_p");

        assert_eq!(session.active_state.as_ref().unwrap().goals.len(), 0);
    }
}
