use crate::ast::{Atomic, Formula, FormulaArena, Var};
use crate::eval::ExecutionLimits;
use crate::budget::ResourceBudget;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct ScopedVar(pub Var, pub usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Constraint {
    pub v1: ScopedVar,
    pub v2: ScopedVar,
    pub weight: i32,
    pub in_comp: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Edge {
    pub source: ScopedVar,
    pub target: ScopedVar,
    pub weight: i32,
    pub in_comp: bool,
}

impl From<Constraint> for Edge {
    fn from(c: Constraint) -> Self {
        Edge {
            source: c.v1,
            target: c.v2,
            weight: c.weight,
            in_comp: c.in_comp,
        }
    }
}

pub fn extract_atom_constraints(
    atomic: &Atomic,
    depth: usize,
    in_comp: bool,
    edge_count: &mut usize,
) -> Vec<Constraint> {
    let mut constraints = Vec::new();
    match atomic {
        Atomic::Eq(x, y) => {
            let sx = ScopedVar(x.clone(), depth);
            let sy = ScopedVar(y.clone(), depth);
            constraints.push(Constraint {
                v1: sx.clone(),
                v2: sy.clone(),
                weight: 0,
                in_comp,
            });
            constraints.push(Constraint {
                v1: sy,
                v2: sx,
                weight: 0,
                in_comp,
            });
            *edge_count += 2;
        }
        Atomic::Mem(x, y) => {
            let sx = ScopedVar(x.clone(), depth);
            let sy = ScopedVar(y.clone(), depth);
            constraints.push(Constraint {
                v1: sx.clone(),
                v2: sy.clone(),
                weight: 1,
                in_comp,
            });
            constraints.push(Constraint {
                v1: sy,
                v2: sx,
                weight: -1,
                in_comp,
            });
            *edge_count += 2;
        }
        Atomic::Lt(x, y) => {
            let sx = ScopedVar(x.clone(), depth);
            let sy = ScopedVar(y.clone(), depth);
            constraints.push(Constraint {
                v1: sy.clone(),
                v2: sx.clone(),
                weight: -1,
                in_comp,
            });
            *edge_count += 1;
        }
        Atomic::QPair(p, x, y) => {
            let sp = ScopedVar(p.clone(), depth);
            let sx = ScopedVar(x.clone(), depth);
            let sy = ScopedVar(y.clone(), depth);
            constraints.push(Constraint { v1: sp.clone(), v2: sx.clone(), weight: 0, in_comp });
            constraints.push(Constraint { v1: sx, v2: sp.clone(), weight: 0, in_comp });
            constraints.push(Constraint { v1: sp.clone(), v2: sy.clone(), weight: 0, in_comp });
            constraints.push(Constraint { v1: sy, v2: sp, weight: 0, in_comp });
            *edge_count += 4;
        }
        Atomic::QProj1(x, p) => {
            let sx = ScopedVar(x.clone(), depth);
            let sp = ScopedVar(p.clone(), depth);
            constraints.push(Constraint { v1: sx.clone(), v2: sp.clone(), weight: 0, in_comp });
            constraints.push(Constraint { v1: sp, v2: sx, weight: 0, in_comp });
            *edge_count += 2;
        }
        Atomic::QProj2(y, p) => {
            let sy = ScopedVar(y.clone(), depth);
            let sp = ScopedVar(p.clone(), depth);
            constraints.push(Constraint { v1: sy.clone(), v2: sp.clone(), weight: 0, in_comp });
            constraints.push(Constraint { v1: sp, v2: sy, weight: 0, in_comp });
            *edge_count += 2;
        }
        Atomic::App(z, u, v) => {
            let sz = ScopedVar(z.clone(), depth);
            let su = ScopedVar(u.clone(), depth);
            let sv = ScopedVar(v.clone(), depth);
            constraints.push(Constraint { v1: sz.clone(), v2: sv.clone(), weight: 0, in_comp });
            constraints.push(Constraint { v1: sv, v2: sz.clone(), weight: 0, in_comp });
            constraints.push(Constraint { v1: sz.clone(), v2: su.clone(), weight: 0, in_comp });
            constraints.push(Constraint { v1: su, v2: sz, weight: 0, in_comp });
            *edge_count += 4;
        }
        Atomic::Lam(z, x, t) => {
            let sz = ScopedVar(z.clone(), depth);
            let sx = ScopedVar(x.clone(), depth);
            let st = ScopedVar(t.clone(), depth);
            constraints.push(Constraint { v1: sz.clone(), v2: st.clone(), weight: 0, in_comp });
            constraints.push(Constraint { v1: st, v2: sz.clone(), weight: 0, in_comp });
            constraints.push(Constraint { v1: sz.clone(), v2: sx.clone(), weight: 0, in_comp });
            constraints.push(Constraint { v1: sx, v2: sz, weight: 0, in_comp });
            *edge_count += 4;
        }
    }
    constraints
}

pub fn extract_dnf_clauses(
    arena: &FormulaArena,
    formula_idx: usize,
    depth: usize,
    in_comp: bool,
    budget: &ResourceBudget,
    edge_count: &mut usize,
) -> Vec<Vec<Constraint>> {
    extract_dnf_clauses_aux(arena, formula_idx, false, depth, in_comp, budget, edge_count)
}

pub fn extract_dnf_clauses_aux(
    arena: &FormulaArena,
    formula_idx: usize,
    is_negated: bool,
    depth: usize,
    in_comp: bool,
    budget: &ResourceBudget,
    edge_count: &mut usize,
) -> Vec<Vec<Constraint>> {
    if depth > budget.max_depth {
        panic!("Graph Extraction Nesting Limit Exceeded");
    }

    let formula = match arena.get(formula_idx) {
        Some(f) => f,
        None => return vec![Vec::new()],
    };

    if !is_negated {
        match formula {
            Formula::Atom(atomic) => {
                vec![extract_atom_constraints(atomic, depth, in_comp, edge_count)]
            }
            Formula::Neg(f_idx) => {
                extract_dnf_clauses_aux(arena, *f_idx, true, depth, in_comp, budget, edge_count)
            }
            Formula::Conj(f1_idx, f2_idx) => {
                let left_clauses = extract_dnf_clauses_aux(arena, *f1_idx, false, depth, in_comp, budget, edge_count);
                let right_clauses = extract_dnf_clauses_aux(arena, *f2_idx, false, depth, in_comp, budget, edge_count);
                let mut combined = Vec::new();
                for lc in &left_clauses {
                    for rc in &right_clauses {
                        let mut merged = lc.clone();
                        merged.extend(rc.clone());
                        combined.push(merged);
                    }
                }
                if combined.is_empty() {
                    left_clauses
                } else {
                    combined
                }
            }
            Formula::Disj(f1_idx, f2_idx) => {
                let mut clauses = extract_dnf_clauses_aux(arena, *f1_idx, false, depth, in_comp, budget, edge_count);
                clauses.extend(extract_dnf_clauses_aux(arena, *f2_idx, false, depth, in_comp, budget, edge_count));
                clauses
            }
            Formula::Impl(f1_idx, f2_idx) => {
                // A -> B => ~A \/ B
                let mut clauses = extract_dnf_clauses_aux(arena, *f1_idx, true, depth, in_comp, budget, edge_count);
                clauses.extend(extract_dnf_clauses_aux(arena, *f2_idx, false, depth, in_comp, budget, edge_count));
                clauses
            }
            Formula::Univ(_, _, f_idx) | Formula::Exist(_, _, f_idx) => {
                extract_dnf_clauses_aux(arena, *f_idx, false, depth + 1, in_comp, budget, edge_count)
            }
            Formula::Comp(_, _, f_idx) => {
                extract_dnf_clauses_aux(arena, *f_idx, false, depth + 1, true, budget, edge_count)
            }
        }
    } else {
        // Negated formula branch
        match formula {
            Formula::Atom(atomic) => {
                vec![extract_atom_constraints(atomic, depth, in_comp, edge_count)]
            }
            Formula::Neg(f_idx) => {
                // Double negation: ~~A => A
                extract_dnf_clauses_aux(arena, *f_idx, false, depth, in_comp, budget, edge_count)
            }
            Formula::Conj(f1_idx, f2_idx) => {
                // De Morgan: ~(A /\ B) => ~A \/ ~B
                let mut clauses = extract_dnf_clauses_aux(arena, *f1_idx, true, depth, in_comp, budget, edge_count);
                clauses.extend(extract_dnf_clauses_aux(arena, *f2_idx, true, depth, in_comp, budget, edge_count));
                clauses
            }
            Formula::Disj(f1_idx, f2_idx) => {
                // De Morgan: ~(A \/ B) => ~A /\ ~B
                let left_clauses = extract_dnf_clauses_aux(arena, *f1_idx, true, depth, in_comp, budget, edge_count);
                let right_clauses = extract_dnf_clauses_aux(arena, *f2_idx, true, depth, in_comp, budget, edge_count);
                let mut combined = Vec::new();
                for lc in &left_clauses {
                    for rc in &right_clauses {
                        let mut merged = lc.clone();
                        merged.extend(rc.clone());
                        combined.push(merged);
                    }
                }
                if combined.is_empty() {
                    left_clauses
                } else {
                    combined
                }
            }
            Formula::Impl(f1_idx, f2_idx) => {
                // ~(A -> B) => A /\ ~B
                let left_clauses = extract_dnf_clauses_aux(arena, *f1_idx, false, depth, in_comp, budget, edge_count);
                let right_clauses = extract_dnf_clauses_aux(arena, *f2_idx, true, depth, in_comp, budget, edge_count);
                let mut combined = Vec::new();
                for lc in &left_clauses {
                    for rc in &right_clauses {
                        let mut merged = lc.clone();
                        merged.extend(rc.clone());
                        combined.push(merged);
                    }
                }
                if combined.is_empty() {
                    left_clauses
                } else {
                    combined
                }
            }
            Formula::Univ(_, _, f_idx) | Formula::Exist(_, _, f_idx) => {
                extract_dnf_clauses_aux(arena, *f_idx, true, depth + 1, in_comp, budget, edge_count)
            }
            Formula::Comp(_, _, f_idx) => {
                extract_dnf_clauses_aux(arena, *f_idx, true, depth + 1, true, budget, edge_count)
            }
        }
    }
}

pub fn extract_constraints_aux(
    arena: &FormulaArena,
    formula_idx: usize,
    depth: usize,
    in_comp: bool,
    budget: &ResourceBudget,
    edge_count: &mut usize,
) -> Vec<Constraint> {
    let clauses = extract_dnf_clauses(arena, formula_idx, depth, in_comp, budget, edge_count);
    if let Some(first) = clauses.into_iter().next() {
        first
    } else {
        Vec::new()
    }
}

/// The GraphArena represents the CPU Geometry Layer in the hybrid pipeline.
/// It translates the semantic interactions (from the `FormulaArena`) into a weighted directed graph
/// using De Bruijn indexing and lexical depths, enabling purely structural verification.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GraphArena {
    pub vars: Vec<ScopedVar>,
    pub var_to_idx: HashMap<ScopedVar, usize>,
    pub edges: Vec<(usize, usize, i32, bool)>, // Added in_comp
}

impl GraphArena {
    pub fn new() -> Self {
        Self {
            vars: Vec::new(),
            var_to_idx: HashMap::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_var(&mut self, var: ScopedVar) -> usize {
        if let Some(&idx) = self.var_to_idx.get(&var) {
            idx
        } else {
            let idx = self.vars.len();
            self.vars.push(var.clone());
            self.var_to_idx.insert(var, idx);
            idx
        }
    }

    pub fn from_constraints(constraints: &[Constraint]) -> Self {
        let mut arena = Self::new();
        for c in constraints {
            let u = arena.add_var(c.v1.clone());
            let v = arena.add_var(c.v2.clone());
            arena.edges.push((u, v, c.weight, c.in_comp));
        }
        arena
    }

    pub fn collapse_scc_0_weight(&mut self) {
        // Obsolete: SCC flattening is now handled natively within evaluate_topology using tarjan_scc.
        // This is kept strictly for CLI compatibility to avoid refactoring the CLI arguments at this moment.
    }

    /// Returns Strongly Connected Components for 0-weight edges using Tarjan's single-pass algorithm.
    pub fn tarjan_scc(&self) -> Vec<Vec<usize>> {
        let n = self.vars.len();
        if n == 0 {
            return Vec::new();
        }

        let mut adj = vec![Vec::new(); n];
        for &(u, v, w, _) in &self.edges {
            if w == 0 {
                adj[u].push(v);
            }
        }

        let mut dfn = vec![None; n];
        let mut low = vec![0; n];
        let mut on_stack = vec![false; n];
        let mut stack = Vec::new();
        let mut timer = 0;
        let mut sccs = Vec::new();

        fn dfs(
            u: usize,
            adj: &[Vec<usize>],
            dfn: &mut [Option<usize>],
            low: &mut [usize],
            on_stack: &mut [bool],
            stack: &mut Vec<usize>,
            timer: &mut usize,
            sccs: &mut Vec<Vec<usize>>,
        ) {
            dfn[u] = Some(*timer);
            low[u] = *timer;
            *timer += 1;
            stack.push(u);
            on_stack[u] = true;

            for &v in &adj[u] {
                if dfn[v].is_none() {
                    dfs(v, adj, dfn, low, on_stack, stack, timer, sccs);
                    low[u] = low[u].min(low[v]);
                } else if on_stack[v] {
                    low[u] = low[u].min(dfn[v].unwrap());
                }
            }

            if low[u] == dfn[u].unwrap() {
                let mut scc = Vec::new();
                while let Some(v) = stack.pop() {
                    on_stack[v] = false;
                    scc.push(v);
                    if v == u {
                        break;
                    }
                }
                sccs.push(scc);
            }
        }

        for i in 0..n {
            if dfn[i].is_none() {
                dfs(i, &adj, &mut dfn, &mut low, &mut on_stack, &mut stack, &mut timer, &mut sccs);
            }
        }

        sccs
    }

    /// Alias to tarjan_scc for backwards compatibility
    pub fn kosaraju_scc(&self) -> Vec<Vec<usize>> {
        self.tarjan_scc()
    }

    pub fn contract_graph(&self, sccs: &[Vec<usize>]) -> (Vec<usize>, Vec<(usize, usize, i32)>, Vec<usize>) {
        let n = self.vars.len();
        let mut rep = vec![0; n];
        for scc in sccs {
            let r = *scc.iter().min().unwrap_or(&0);
            for &u in scc {
                rep[u] = r;
            }
        }

        let mut c_vars = Vec::new();
        let mut c_vars_set = std::collections::HashSet::new();
        for &r in &rep {
            if c_vars_set.insert(r) {
                c_vars.push(r);
            }
        }

        let mut c_edges = std::collections::HashSet::new();
        for &(u, v, w, _) in &self.edges {
            let ru = rep[u];
            let rv = rep[v];
            if ru != rv || w != 0 {
                c_edges.insert((ru, rv, w));
            }
        }

        (c_vars, c_edges.into_iter().collect(), rep)
    }

    /// Continuous daemon that isolates Strongly Cantorian (ZFC-compliant) bedrock
    /// by scanning for subgraphs satisfying the x = T(x) constraint (topological self-loops)
    /// and severing their outgoing +1 offset edges to reduce computational load.
    pub fn isolate_sc_bedrock(&mut self) -> Vec<String> {
        let mut sc_nodes = HashSet::new();

        // Detect x = T(x) constraints: nodes that have a +1 or -1 weight self-loop.
        // We strictly enforce that Comprehension boundaries are respected:
        // if the self-loop is part of a Comprehension (in_comp == true), it is an
        // unstratifiable paradox and MUST NOT be isolated as Strongly Cantorian bedrock.
        for &(u, v, w, in_comp) in &self.edges {
            if u == v && w != 0 && !in_comp {
                sc_nodes.insert(u);
            }
        }

        if sc_nodes.is_empty() {
            return Vec::new();
        }

        let mut actions = Vec::new();
        let mut new_edges = HashSet::new();

        for &(u, v, w, in_comp) in &self.edges {
            if u == v && w != 0 && !in_comp {
                actions.push(format!(
                    "Neutralized SC defining self-loop on {}",
                    self.var_name(u)
                ));
                continue;
            }
            // Only sever connections if they are NOT inside a Comprehension
            if sc_nodes.contains(&u) && w == 1 && !in_comp {
                actions.push(format!(
                    "Severed outgoing +1 offset edge from SC bedrock node {} to {}",
                    self.var_name(u),
                    self.var_name(v)
                ));
                continue;
            }
            if sc_nodes.contains(&v) && w == -1 && !in_comp {
                actions.push(format!(
                    "Severed incoming -1 offset edge to SC bedrock node {} from {}",
                    self.var_name(v),
                    self.var_name(u)
                ));
                continue;
            }
            new_edges.insert((u, v, w, in_comp));
        }
        self.edges = new_edges.into_iter().collect();

        // Remove duplicates and return
        let mut unique_actions: Vec<String> = actions
            .into_iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
        unique_actions.sort();
        unique_actions
    }

    fn var_name(&self, u: usize) -> String {
        let var = &self.vars[u];
        let name = match &var.0 {
            crate::ast::Var::Free(n) => n.clone(),
            crate::ast::Var::Bound(idx) => format!("b{}", idx),
        };
        format!("{}_{}", name, var.1)
    }


    pub fn topological_sort(&self) -> Option<Vec<usize>> {
        let n = self.vars.len();
        let mut in_degree = vec![0; n];
        let mut adj = vec![Vec::new(); n];

        for &(u, v, _, _) in &self.edges {
            adj[u].push(v);
            in_degree[v] += 1;
        }

        let mut queue = std::collections::VecDeque::new();
        for i in 0..n {
            if in_degree[i] == 0 {
                queue.push_back(i);
            }
        }

        let mut order = Vec::new();
        while let Some(u) = queue.pop_front() {
            order.push(u);
            for &v in &adj[u] {
                in_degree[v] -= 1;
                if in_degree[v] == 0 {
                    queue.push_back(v);
                }
            }
        }

        if order.len() == n {
            Some(order)
        } else {
            None
        }
    }

    pub fn classify_subsystems(&self, d: &[i32]) -> (bool, bool) {
        let mut base_weight = i32::MIN;
        for (i, var) in self.vars.iter().enumerate() {
            if let crate::ast::Var::Free(_) = var.0 {
                if d[i] > base_weight {
                    base_weight = d[i];
                }
            }
        }

        if base_weight == i32::MIN {
            for &w in d {
                if w > base_weight {
                    base_weight = w;
                }
            }
            if base_weight == i32::MIN {
                base_weight = 0;
            }
        }

        let mut is_nfi = true;
        let mut is_nfp = true;

        for (i, var) in self.vars.iter().enumerate() {
            let weight = d[i];
            
            if weight > base_weight + 1 {
                is_nfi = false;
            }

            match var.0 {
                crate::ast::Var::Free(_) => {
                    if weight > base_weight + 1 {
                        is_nfp = false;
                    }
                }
                crate::ast::Var::Bound(_) => {
                    if weight > base_weight {
                        is_nfp = false;
                    }
                }
            }
        }

        (is_nfp, is_nfi)
    }

    /// Evaluates the topological structure using a hybrid approach.
    /// It attempts a fast O(V+E) DAG Shortest Path evaluation first. If the graph contains 
    /// cycles, it falls back to the O(V*E) Bellman-Ford algorithm to detect negative-weight cycles
    /// (Extensionality Collisions).
    pub fn evaluate_topology(&mut self) -> Result<(Vec<i32>, Vec<String>, bool, bool), String> {
        // Run the continuous daemon to dynamically sever outgoing +1 offset edges from SC bedrock
        let sc_actions = self.isolate_sc_bedrock();

        let n = self.vars.len();
        if n == 0 {
            return Ok((Vec::new(), sc_actions, true, true));
        }

        let sccs = self.tarjan_scc();
        let (c_vars, c_edges, reps) = self.contract_graph(&sccs);

        let mut in_degree = HashMap::new();
        for &u in &c_vars {
            in_degree.insert(u, 0);
        }
        let mut adj = HashMap::new();
        for &(u, v, w) in &c_edges {
            adj.entry(u).or_insert_with(Vec::new).push((v, w));
            *in_degree.entry(v).or_insert(0) += 1;
        }

        let mut queue = std::collections::VecDeque::new();
        for (&u, &deg) in &in_degree {
            if deg == 0 {
                queue.push_back(u);
            }
        }

        let mut order = Vec::new();
        while let Some(u) = queue.pop_front() {
            order.push(u);
            if let Some(neighbors) = adj.get(&u) {
                for &(v, _) in neighbors {
                    if let Some(deg) = in_degree.get_mut(&v) {
                        *deg -= 1;
                        if *deg == 0 {
                            queue.push_back(v);
                        }
                    }
                }
            }
        }

        // Fast-path: O(V+E) DAG Shortest Path on Contracted Graph
        if order.len() == c_vars.len() {
            let mut c_d = HashMap::new();
            for &u in &c_vars {
                c_d.insert(u, 0);
            }
            for &u in &order {
                let du = *c_d.get(&u).unwrap();
                if let Some(neighbors) = adj.get(&u) {
                    for &(v, w) in neighbors {
                        let dv = *c_d.get(&v).unwrap();
                        if du + w < dv {
                            c_d.insert(v, du + w);
                        }
                    }
                }
            }

            let mut d = vec![0; n];
            for i in 0..n {
                d[i] = *c_d.get(&reps[i]).unwrap();
            }

            let (is_nfp, is_nfi) = self.classify_subsystems(&d);
            return Ok((d, sc_actions, is_nfp, is_nfi));
        }

        // Fallback: O(V*E) Bellman-Ford
        let mut d = vec![0; n];
        let mut p: Vec<Option<(usize, i32)>> = vec![None; n];

        // Relax edges n-1 times
        for _ in 0..n {
            let mut changed = false;
            for &(u, v, w, _) in &self.edges {
                if d[u] + w < d[v] {
                    d[v] = d[u] + w;
                    p[v] = Some((u, w));
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }

        // Final pass for negative weight cycles
        let mut collision_vertex = None;
        for &(u, v, w, _) in &self.edges {
            if d[u] + w < d[v] {
                collision_vertex = Some(v);
                p[v] = Some((u, w));
                break;
            }
        }

        if let Some(mut curr) = collision_vertex {
            let lambda_star = match ExecutionLimits::compute_for_graph(self) {
                Some(limits) => limits.mcm,
                None => f64::NEG_INFINITY,
            };

            for _ in 0..n {
                curr = p[curr].unwrap().0;
            }

            let cycle_start = curr;
            let mut cycle = Vec::new();

            loop {
                let (prev, w) = p[curr].unwrap();
                cycle.push((prev, curr, w));
                curr = prev;
                if curr == cycle_start {
                    break;
                }
            }

            cycle.reverse();

            let mut result = String::new();
            result.push_str(&format!("Extensionality Collision: Negative-weight cycle detected (μ* = {:.4})!\n", lambda_star));
            result.push_str("Engine halted safely (K_ITERATION_HALT)\n");
            result.push_str("Topological Trace: ");

            let mut sum_str = Vec::new();
            let mut total_weight = 0;
            for (u, v, w) in &cycle {
                let u_var = &self.vars[*u];
                let v_var = &self.vars[*v];

                let u_name = match &u_var.0 {
                    crate::ast::Var::Free(name) => name.clone(),
                    crate::ast::Var::Bound(idx) => format!("b{}", idx),
                };
                let v_name = match &v_var.0 {
                    crate::ast::Var::Free(name) => name.clone(),
                    crate::ast::Var::Bound(idx) => format!("b{}", idx),
                };

                let u_str = format!("{}_{}", u_name, u_var.1);
                let v_str = format!("{}_{}", v_name, v_var.1);

                sum_str.push(format!("{} -> {} ({})", u_str, v_str, w));
                total_weight += w;
            }

            result.push_str(&sum_str.join(" + "));
            result.push_str(&format!(" = {}", total_weight));

            return Err(result);
        }

        let (is_nfp, is_nfi) = self.classify_subsystems(&d);
        Ok((d, sc_actions, is_nfp, is_nfi))
    }

    /// Extract Minimal Conflict Clauses for Vector Superposition (IDL Masking)
    /// When Bellman-Ford flags a negative-weight cycle, this identifies the nodes
    /// involved so the upper ingestion layer can translate them into a hyperdimensional 
    /// destructive interference mask.
    pub fn extract_conflict_clauses(&mut self) -> Vec<Vec<usize>> {
        let n = self.vars.len();
        let mut d = vec![0; n];
        let mut p: Vec<Option<(usize, i32)>> = vec![None; n];
        
        // Relax edges
        for _ in 0..n {
            for &(u, v, w, _) in &self.edges {
                if d[u] + w < d[v] {
                    d[v] = d[u] + w;
                    p[v] = Some((u, w));
                }
            }
        }
        
        let mut conflict_clauses = Vec::new();
        // Detect cycle
        for &(u, v, w, _) in &self.edges {
            if d[u] + w < d[v] {
                // We found a node 'v' in a negative weight cycle
                let mut curr = v;
                for _ in 0..n {
                    if let Some((prev, _)) = p[curr] {
                        curr = prev;
                    }
                }
                
                let cycle_start = curr;
                let mut cycle = Vec::new();
                
                loop {
                    if let Some((prev, _)) = p[curr] {
                        cycle.push(curr);
                        curr = prev;
                        if curr == cycle_start {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                cycle.reverse();
                
                // Only add if not already present
                let mut sorted_cycle = cycle.clone();
                sorted_cycle.sort();
                
                let is_duplicate = conflict_clauses.iter().any(|c: &Vec<usize>| {
                    let mut sc = c.clone();
                    sc.sort();
                    sc == sorted_cycle
                });
                
                if !is_duplicate {
                    conflict_clauses.push(cycle);
                }
            }
        }
        
        conflict_clauses
    }

    /// Evaluates a formula in Disjunctive Normal Form (DNF) across all extracted clauses.
    /// Stratification succeeds if at least one DNF clause forms an acyclic/valid topological graph.
    pub fn evaluate_dnf_formula(
        arena: &FormulaArena,
        formula_idx: usize,
        budget: &ResourceBudget,
    ) -> Result<(Vec<i32>, Vec<String>, bool, bool), String> {
        let mut edge_count = 0;
        let clauses = extract_dnf_clauses(arena, formula_idx, 0, false, budget, &mut edge_count);
        let mut last_err = String::from("No DNF clauses generated");
        for clause in clauses {
            let mut graph = GraphArena::from_constraints(&clause);
            match graph.evaluate_topology() {
                Ok(res) => return Ok(res),
                Err(e) => last_err = e,
            }
        }
        Err(last_err)
    }
}
