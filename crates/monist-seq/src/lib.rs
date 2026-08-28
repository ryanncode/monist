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
}
