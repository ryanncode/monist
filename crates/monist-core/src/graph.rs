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

pub fn extract_constraints_aux(
    arena: &FormulaArena,
    formula_idx: usize,
    depth: usize,
    in_comp: bool,
    budget: &ResourceBudget,
    edge_count: &mut usize,
) -> Vec<Constraint> {
    if depth > budget.max_depth {
        panic!("Graph Extraction Nesting Limit Exceeded");
    }
    let mut constraints = Vec::new();

    let formula = match arena.get(formula_idx) {
        Some(f) => f,
        None => return constraints,
    };

    match formula {
        Formula::Atom(atomic) => match atomic {
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
            _ => {}
        },
        Formula::Neg(f_idx) => {
            constraints.extend(extract_constraints_aux(arena, *f_idx, depth, in_comp, budget, edge_count));
        }
        Formula::Conj(f1_idx, f2_idx)
        | Formula::Disj(f1_idx, f2_idx)
        | Formula::Impl(f1_idx, f2_idx) => {
            constraints.extend(extract_constraints_aux(arena, *f1_idx, depth, in_comp, budget, edge_count));
            constraints.extend(extract_constraints_aux(arena, *f2_idx, depth, in_comp, budget, edge_count));
        }
        Formula::Univ(_, _, f_idx) | Formula::Exist(_, _, f_idx) => {
            constraints.extend(extract_constraints_aux(arena, *f_idx, depth + 1, in_comp, budget, edge_count));
        }
        Formula::Comp(_, _, f_idx) => {
            constraints.extend(extract_constraints_aux(arena, *f_idx, depth + 1, true, budget, edge_count));
        }
    }
    if *edge_count > budget.max_graph_edges {
        panic!("Graph Edge Limit Exceeded");
    }
    constraints
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

    /// Implement Tarjan's SCC algorithm to locate and safely collapse 0-weight semantic cycles
    pub fn collapse_scc_0_weight(&mut self) {
        let n = self.vars.len();
        if n == 0 {
            return;
        }

        let mut adj = vec![Vec::new(); n];
        for &(u, v, w, _) in &self.edges {
            if w == 0 {
                adj[u].push(v);
            }
        }

        struct Tarjan<'a> {
            adj: &'a [Vec<usize>],
            index: usize,
            indices: Vec<Option<usize>>,
            lowlinks: Vec<usize>,
            on_stack: Vec<bool>,
            stack: Vec<usize>,
            scc_count: usize,
            component: Vec<usize>,
        }

        impl<'a> Tarjan<'a> {
            fn strongconnect(&mut self, v: usize) {
                self.indices[v] = Some(self.index);
                self.lowlinks[v] = self.index;
                self.index += 1;
                self.stack.push(v);
                self.on_stack[v] = true;

                for &w in &self.adj[v] {
                    if self.indices[w].is_none() {
                        self.strongconnect(w);
                        self.lowlinks[v] = self.lowlinks[v].min(self.lowlinks[w]);
                    } else if self.on_stack[w] {
                        self.lowlinks[v] = self.lowlinks[v].min(self.indices[w].unwrap());
                    }
                }

                if self.lowlinks[v] == self.indices[v].unwrap() {
                    loop {
                        let w = self.stack.pop().unwrap();
                        self.on_stack[w] = false;
                        self.component[w] = self.scc_count;
                        if w == v {
                            break;
                        }
                    }
                    self.scc_count += 1;
                }
            }
        }

        let mut tarjan = Tarjan {
            adj: &adj,
            index: 0,
            indices: vec![None; n],
            lowlinks: vec![0; n],
            on_stack: vec![false; n],
            stack: Vec::new(),
            scc_count: 0,
            component: vec![0; n],
        };

        for i in 0..n {
            if tarjan.indices[i].is_none() {
                tarjan.strongconnect(i);
            }
        }

        let scc_count = tarjan.scc_count;
        let component = tarjan.component;

        // Map components to the smallest representative variable in that component
        let mut reps = vec![n; scc_count];
        for i in 0..n {
            let comp = component[i];
            if i < reps[comp] {
                reps[comp] = i;
            }
        }

        // Update edges to use representatives
        let mut new_edges = HashSet::new();

        // Ensure that collapsed variables are not orphaned in the SMT matrix.
        // We mathematically assert their equivalence to the Strongly Cantorian representative.
        for i in 0..n {
            let rep = reps[component[i]];
            if i != rep {
                new_edges.insert((rep, i, 0, false));
                new_edges.insert((i, rep, 0, false));
            }
        }

        for &(u, v, w, in_comp) in &self.edges {
            let rep_u = reps[component[u]];
            let rep_v = reps[component[v]];
            if rep_u != rep_v || w != 0 {
                new_edges.insert((rep_u, rep_v, w, in_comp));
            }
        }
        self.edges = new_edges.into_iter().collect();
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

        // Fast-path: O(V+E) DAG Shortest Path
        if let Some(order) = self.topological_sort() {
            let mut d = vec![0; n];
            
            let mut adj = vec![Vec::new(); n];
            for &(u, v, w, _) in &self.edges {
                adj[u].push((v, w));
            }
            
            for &u in &order {
                for &(v, w) in &adj[u] {
                    if d[u] + w < d[v] {
                        d[v] = d[u] + w;
                    }
                }
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
}
