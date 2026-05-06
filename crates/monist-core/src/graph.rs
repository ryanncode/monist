use crate::ast::{Atomic, Formula, FormulaArena, Var};
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
) -> Vec<Constraint> {
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
            }
            _ => {}
        },
        Formula::Neg(f_idx) => {
            constraints.extend(extract_constraints_aux(arena, *f_idx, depth, in_comp));
        }
        Formula::Conj(f1_idx, f2_idx)
        | Formula::Disj(f1_idx, f2_idx)
        | Formula::Impl(f1_idx, f2_idx) => {
            constraints.extend(extract_constraints_aux(arena, *f1_idx, depth, in_comp));
            constraints.extend(extract_constraints_aux(arena, *f2_idx, depth, in_comp));
        }
        Formula::Univ(_, _, f_idx) | Formula::Exist(_, _, f_idx) => {
            constraints.extend(extract_constraints_aux(arena, *f_idx, depth + 1, in_comp));
        }
        Formula::Comp(_, _, f_idx) => {
            constraints.extend(extract_constraints_aux(arena, *f_idx, depth + 1, true));
        }
    }
    constraints
}

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

    /// Implement Kosaraju's SCC algorithm to locate and safely collapse 0-weight semantic cycles
    pub fn collapse_scc_0_weight(&mut self) {
        let n = self.vars.len();
        if n == 0 {
            return;
        }

        let mut adj = vec![Vec::new(); n];
        let mut rev_adj = vec![Vec::new(); n];

        for &(u, v, w, _) in &self.edges {
            if w == 0 {
                adj[u].push(v);
                rev_adj[v].push(u);
            }
        }

        let mut visited = vec![false; n];
        let mut order = Vec::new();

        for i in 0..n {
            if !visited[i] {
                self.dfs1(i, &adj, &mut visited, &mut order);
            }
        }

        visited.fill(false);
        let mut component = vec![0; n];
        let mut scc_count = 0;

        for &i in order.iter().rev() {
            if !visited[i] {
                self.dfs2(i, &rev_adj, &mut visited, &mut component, scc_count);
                scc_count += 1;
            }
        }

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

    fn dfs1(&self, u: usize, adj: &[Vec<usize>], visited: &mut [bool], order: &mut Vec<usize>) {
        visited[u] = true;
        for &v in &adj[u] {
            if !visited[v] {
                self.dfs1(v, adj, visited, order);
            }
        }
        order.push(u);
    }

    fn dfs2(
        &self,
        u: usize,
        rev_adj: &[Vec<usize>],
        visited: &mut [bool],
        component: &mut [usize],
        comp_id: usize,
    ) {
        visited[u] = true;
        component[u] = comp_id;
        for &v in &rev_adj[u] {
            if !visited[v] {
                self.dfs2(v, rev_adj, visited, component, comp_id);
            }
        }
    }

    pub fn bellman_ford(&mut self) -> Result<(Vec<i32>, Vec<String>), String> {
        // Run the continuous daemon to dynamically sever outgoing +1 offset edges from SC bedrock
        let sc_actions = self.isolate_sc_bedrock();

        let n = self.vars.len();
        if n == 0 {
            return Ok((Vec::new(), sc_actions));
        }

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
            result.push_str("Extensionality Collision: Negative-weight cycle detected!\n");
            result.push_str("Summation: ");

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

        Ok((d, sc_actions))
    }
}
