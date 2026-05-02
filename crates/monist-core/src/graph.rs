use crate::ast::{Atomic, Formula, FormulaArena, Var};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ScopedVar(pub Var, pub usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Constraint {
    pub v1: ScopedVar,
    pub v2: ScopedVar,
    pub weight: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Edge {
    pub source: ScopedVar,
    pub target: ScopedVar,
    pub weight: i32,
}

impl From<Constraint> for Edge {
    fn from(c: Constraint) -> Self {
        Edge {
            source: c.v1,
            target: c.v2,
            weight: c.weight,
        }
    }
}

pub fn extract_constraints_aux(
    arena: &FormulaArena,
    formula_idx: usize,
    depth: usize,
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
                constraints.push(Constraint { v1: sx.clone(), v2: sy.clone(), weight: 0 });
                constraints.push(Constraint { v1: sy, v2: sx, weight: 0 });
            }
            Atomic::Mem(x, y) => {
                let sx = ScopedVar(x.clone(), depth);
                let sy = ScopedVar(y.clone(), depth);
                constraints.push(Constraint { v1: sx.clone(), v2: sy.clone(), weight: 1 });
                constraints.push(Constraint { v1: sy, v2: sx, weight: -1 });
            }
            _ => {}
        },
        Formula::Neg(f_idx) => {
            constraints.extend(extract_constraints_aux(arena, *f_idx, depth));
        }
        Formula::Conj(f1_idx, f2_idx) | Formula::Disj(f1_idx, f2_idx) | Formula::Impl(f1_idx, f2_idx) => {
            constraints.extend(extract_constraints_aux(arena, *f1_idx, depth));
            constraints.extend(extract_constraints_aux(arena, *f2_idx, depth));
        }
        Formula::Univ(_, _, f_idx) | Formula::Comp(_, _, f_idx) => {
            constraints.extend(extract_constraints_aux(arena, *f_idx, depth + 1));
        }
    }
    constraints
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GraphArena {
    pub vars: Vec<ScopedVar>,
    pub var_to_idx: HashMap<ScopedVar, usize>,
    pub edges: Vec<(usize, usize, i32)>,
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
            arena.edges.push((u, v, c.weight));
        }
        arena
    }

    /// Implement Kosaraju's SCC algorithm to locate and safely collapse 0-weight semantic cycles
    pub fn collapse_scc_0_weight(&mut self) {
        let n = self.vars.len();
        if n == 0 { return; }

        let mut adj = vec![Vec::new(); n];
        let mut rev_adj = vec![Vec::new(); n];

        for &(u, v, w) in &self.edges {
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
        for &(u, v, w) in &self.edges {
            let rep_u = reps[component[u]];
            let rep_v = reps[component[v]];
            if rep_u != rep_v || w != 0 {
                new_edges.insert((rep_u, rep_v, w));
            }
        }
        self.edges = new_edges.into_iter().collect();
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

    fn dfs2(&self, u: usize, rev_adj: &[Vec<usize>], visited: &mut [bool], component: &mut [usize], comp_id: usize) {
        visited[u] = true;
        component[u] = comp_id;
        for &v in &rev_adj[u] {
            if !visited[v] {
                self.dfs2(v, rev_adj, visited, component, comp_id);
            }
        }
    }

    /// Bellman-Ford evaluation engine to definitively detect Extensionality Collisions
    pub fn bellman_ford(&self) -> Result<Vec<i32>, String> {
        let n = self.vars.len();
        if n == 0 {
            return Ok(Vec::new());
        }

        let mut d = vec![0; n];

        // Relax edges n-1 times
        for _ in 0..n {
            let mut changed = false;
            for &(u, v, w) in &self.edges {
                if d[u] + w < d[v] {
                    d[v] = d[u] + w;
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }

        // Final pass for negative weight cycles
        for &(u, v, w) in &self.edges {
            if d[u] + w < d[v] {
                return Err("Extensionality Collision: Negative-weight cycle detected!".to_string());
            }
        }

        Ok(d)
    }
}
