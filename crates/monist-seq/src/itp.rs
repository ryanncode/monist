use monist_core::ast::{Formula, FormulaArena, Atomic, Var};

#[derive(Debug, Clone, serde::Serialize)]
pub struct Goal {
    pub ctx: Vec<(String, usize)>,
    pub target: usize,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ProofState {
    pub goals: Vec<Goal>,
}

#[derive(Debug)]
pub struct ReplSession {
    pub arena: FormulaArena,
    pub active_state: Option<ProofState>,
    pub theorems: Vec<(String, usize)>,
    pub macros: std::collections::HashMap<String, (Vec<String>, usize)>,
}

impl ReplSession {
    pub fn new() -> Self {
        Self {
            arena: FormulaArena::new(),
            active_state: None,
            theorems: Vec::new(),
            macros: std::collections::HashMap::new(),
        }
    }

    pub fn define_macro(&mut self, name: String, params: Vec<String>, formula_str: &str) -> Result<(), String> {
        let mut parser = monist_parser::parser::Parser::with_macros(
            formula_str,
            &mut self.arena,
            Some(&self.macros),
            monist_core::budget::ResourceBudget::default(),
        );
        let root_idx = parser.parse_formula();
        self.macros.insert(name, (params, root_idx));
        Ok(())
    }

    pub fn start_proof(&mut self, _name: String, target: usize) {
        let goal = Goal {
            ctx: self.theorems.clone(),
            target,
        };
        self.active_state = Some(ProofState { goals: vec![goal] });
    }

    pub fn tactic_intro(&mut self, name: String) -> Result<(), String> {
        let state = self.active_state.as_mut().ok_or("No active goals.")?;
        if state.goals.is_empty() {
            return Err("No active goals.".to_string());
        }

        let mut current_goal = state.goals.remove(0);
        let target_formula = self.arena.get(current_goal.target)
            .ok_or("Target formula not found in arena")?.clone();

        match target_formula {
            Formula::Impl(p, q) => {
                current_goal.ctx.push((name, p));
                current_goal.target = q;
                state.goals.insert(0, current_goal);
                Ok(())
            }
            Formula::Univ(_, _, p) => {
                // Simplified intro for ∀: assume a dummy equality
                let eq_atom = Formula::Atom(Atomic::Eq(Var::Free(name.clone()), Var::Free(name.clone())));
                let eq_idx = self.arena.add(eq_atom);
                current_goal.ctx.push((name, eq_idx));
                current_goal.target = p;
                state.goals.insert(0, current_goal);
                Ok(())
            }
            _ => {
                state.goals.insert(0, current_goal);
                Err("intro: goal is not an implication or universal quantifier.".to_string())
            }
        }
    }

    fn structural_eq(arena: &FormulaArena, idx1: usize, idx2: usize) -> bool {
        if idx1 == idx2 { return true; }
        let f1 = match arena.get(idx1) { Some(f) => f, None => return false };
        let f2 = match arena.get(idx2) { Some(f) => f, None => return false };
        
        match (f1, f2) {
            (Formula::Atom(a1), Formula::Atom(a2)) => a1 == a2,
            (Formula::Neg(i1), Formula::Neg(i2)) => Self::structural_eq(arena, *i1, *i2),
            (Formula::Conj(l1, r1), Formula::Conj(l2, r2)) => 
                Self::structural_eq(arena, *l1, *l2) && Self::structural_eq(arena, *r1, *r2),
            (Formula::Disj(l1, r1), Formula::Disj(l2, r2)) => 
                Self::structural_eq(arena, *l1, *l2) && Self::structural_eq(arena, *r1, *r2),
            (Formula::Impl(l1, r1), Formula::Impl(l2, r2)) => 
                Self::structural_eq(arena, *l1, *l2) && Self::structural_eq(arena, *r1, *r2),
            (Formula::Univ(d1, n1, i1), Formula::Univ(d2, n2, i2)) =>
                d1 == d2 && n1 == n2 && Self::structural_eq(arena, *i1, *i2),
            (Formula::Comp(d1, n1, i1), Formula::Comp(d2, n2, i2)) =>
                d1 == d2 && n1 == n2 && Self::structural_eq(arena, *i1, *i2),
            _ => false,
        }
    }

    pub fn tactic_exact(&mut self, name: &str) -> Result<(), String> {
        let state = self.active_state.as_mut().ok_or("No active goals.")?;
        if state.goals.is_empty() {
            return Err("No active goals.".to_string());
        }

        let current_goal = &state.goals[0];
        let hyp = current_goal.ctx.iter().find(|(n, _)| n == name);

        if let Some((_, hyp_idx)) = hyp {
            if Self::structural_eq(&self.arena, *hyp_idx, current_goal.target) {
                state.goals.remove(0);
                Ok(())
            } else {
                Err(format!("exact: hypothesis {} does not match goal exactly.", name))
            }
        } else {
            Err(format!("exact: hypothesis {} not found.", name))
        }
    }

    pub fn tactic_apply(&mut self, name: &str) -> Result<(), String> {
        let state = self.active_state.as_mut().ok_or("No active goals.")?;
        if state.goals.is_empty() {
            return Err("No active goals.".to_string());
        }

        let mut current_goal = state.goals.remove(0);
        let hyp = current_goal.ctx.iter().find(|(n, _)| n == name).map(|(_, i)| *i);

        if let Some(hyp_idx) = hyp {
            let hyp_formula = self.arena.get(hyp_idx).unwrap().clone();
            if let Formula::Impl(p, q) = hyp_formula {
                if Self::structural_eq(&self.arena, q, current_goal.target) {
                    current_goal.target = p;
                    state.goals.insert(0, current_goal);
                    return Ok(());
                } else {
                    state.goals.insert(0, current_goal);
                    return Err(format!("apply: conclusion of {} does not match goal.", name));
                }
            } else {
                state.goals.insert(0, current_goal);
                Err(format!("apply: hypothesis {} is not an implication.", name))
            }
        } else {
            state.goals.insert(0, current_goal);
            Err(format!("apply: hypothesis {} not found.", name))
        }
    }

    pub fn tactic_split(&mut self) -> Result<(), String> {
        let state = self.active_state.as_mut().ok_or("No active goals.")?;
        if state.goals.is_empty() {
            return Err("No active goals.".to_string());
        }

        let mut current_goal = state.goals.remove(0);
        let target_formula = self.arena.get(current_goal.target).unwrap().clone();

        if let Formula::Conj(p, q) = target_formula {
            let mut g1 = current_goal.clone();
            let mut g2 = current_goal;
            g1.target = p;
            g2.target = q;
            state.goals.insert(0, g2);
            state.goals.insert(0, g1);
            Ok(())
        } else {
            state.goals.insert(0, current_goal);
            Err("split: goal is not a conjunction.".to_string())
        }
    }

    pub fn tactic_left(&mut self) -> Result<(), String> {
        let state = self.active_state.as_mut().ok_or("No active goals.")?;
        if state.goals.is_empty() {
            return Err("No active goals.".to_string());
        }

        let mut current_goal = state.goals.remove(0);
        let target_formula = self.arena.get(current_goal.target).unwrap().clone();

        if let Formula::Disj(p, _) = target_formula {
            current_goal.target = p;
            state.goals.insert(0, current_goal);
            Ok(())
        } else {
            state.goals.insert(0, current_goal);
            Err("left: goal is not a disjunction.".to_string())
        }
    }

    pub fn tactic_right(&mut self) -> Result<(), String> {
        let state = self.active_state.as_mut().ok_or("No active goals.")?;
        if state.goals.is_empty() {
            return Err("No active goals.".to_string());
        }

        let mut current_goal = state.goals.remove(0);
        let target_formula = self.arena.get(current_goal.target).unwrap().clone();

        if let Formula::Disj(_, q) = target_formula {
            current_goal.target = q;
            state.goals.insert(0, current_goal);
            Ok(())
        } else {
            state.goals.insert(0, current_goal);
            Err("right: goal is not a disjunction.".to_string())
        }
    }

    pub fn tactic_destruct(&mut self, name: &str, n1: String, n2: String) -> Result<(), String> {
        let state = self.active_state.as_mut().ok_or("No active goals.")?;
        if state.goals.is_empty() {
            return Err("No active goals.".to_string());
        }

        let mut current_goal = state.goals.remove(0);
        let hyp_idx_opt = current_goal.ctx.iter().position(|(n, _)| n == name);

        if let Some(hyp_pos) = hyp_idx_opt {
            let hyp_idx = current_goal.ctx[hyp_pos].1;
            let hyp_formula = self.arena.get(hyp_idx).unwrap().clone();

            match hyp_formula {
                Formula::Conj(p, q) => {
                    current_goal.ctx.remove(hyp_pos);
                    if !n1.is_empty() { current_goal.ctx.push((n1, p)); }
                    if !n2.is_empty() { current_goal.ctx.push((n2, q)); }
                    state.goals.insert(0, current_goal);
                    Ok(())
                }
                Formula::Disj(p, q) => {
                    current_goal.ctx.remove(hyp_pos);
                    let mut g1 = current_goal.clone();
                    let mut g2 = current_goal.clone();
                    if !n1.is_empty() { g1.ctx.push((n1, p)); }
                    if !n2.is_empty() { g2.ctx.push((n2, q)); }
                    state.goals.insert(0, g2);
                    state.goals.insert(0, g1);
                    Ok(())
                }
                _ => {
                    state.goals.insert(0, current_goal);
                    Err(format!("destruct: hypothesis {} is not a conjunction or disjunction.", name))
                }
            }
        } else {
            state.goals.insert(0, current_goal);
            Err(format!("destruct: hypothesis {} not found.", name))
        }
    }

    pub fn tactic_cut(&mut self, formula_str: &str) -> Result<(), String> {
        let state = self.active_state.as_mut().ok_or("No active goals.")?;
        if state.goals.is_empty() {
            return Err("No active goals.".to_string());
        }

        // Parse formula_str into a formula
        let mut parser = monist_parser::parser::Parser::new(formula_str, &mut self.arena, monist_core::budget::ResourceBudget::default());
        let cut_formula_idx = parser.parse_formula();

        // 1. Evaluate topology! 
        let constraints = monist_core::graph::extract_constraints_aux(
            &self.arena,
            cut_formula_idx,
            0,
            false,
            &monist_core::budget::ResourceBudget::default(),
            &mut 0
        );
        let mut graph = monist_core::graph::GraphArena::from_constraints(&constraints);
        graph.collapse_scc_0_weight();
        match graph.evaluate_topology() {
            Err(_) => {
                return Err("Extensionality Collision! The cut formula violently collides with Extensionality bounds (MCM < 0).".to_string());
            }
            Ok(_) => {} // Cut formula is geometrically valid
        }

        let mut current_goal = state.goals.remove(0);

        // We split the proof state into two goals:
        // Goal 1: Prove the cut formula from the current context
        // Goal 2: Prove the original target, with the cut formula added to the context as hypothesis 'H'
        
        let mut g1 = current_goal.clone();
        g1.target = cut_formula_idx; // Prove the cut formula
        
        let mut g2 = current_goal.clone();
        g2.ctx.push(("H".to_string(), cut_formula_idx)); // Assume cut formula

        // g1 goes first, then g2
        state.goals.insert(0, g2);
        state.goals.insert(0, g1);

        Ok(())
    }
}
