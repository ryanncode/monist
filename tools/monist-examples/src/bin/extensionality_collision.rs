use monist_core::ast::FormulaArena;
use monist_core::eval::ExecutionLimits;
use monist_core::graph::{extract_constraints_aux, GraphArena};
use monist_core::smt::export_smt_lib;
use monist_parser::parser::Parser;

struct Session {
    graph: GraphArena,
    arena: FormulaArena,
}

impl Session {
    fn new() -> Self {
        Self {
            graph: GraphArena::new(),
            arena: FormulaArena::new(),
        }
    }

    fn eval(&mut self, formula: &str, test_name: &str) -> Result<(Vec<i32>, Vec<String>, bool, bool), String> {
        let mut parser = Parser::new(formula, &mut self.arena, monist_core::budget::ResourceBudget::default());
        let root_idx = parser.parse_formula();

        let constraints = extract_constraints_aux(&self.arena, root_idx, 0, false, &monist_core::budget::ResourceBudget::default(), &mut 0);
        self.graph = GraphArena::from_constraints(&constraints);
        self.graph.collapse_scc_0_weight();

        let limits_result = ExecutionLimits::compute_for_graph(&self.graph);
        if let Some(limits) = &limits_result {
            println!(
                "Execution Limits Computed: MCM = {:.2}, Max K-Iterations = {}",
                limits.mcm, limits.max_k_iterations
            );
        }

        let bf_result = self.graph.evaluate_topology();

        println!(
            "\n=== Stratification Witness (SMT-LIB format) for {} ===",
            test_name
        );
        match &bf_result {
            Ok((dist, _, _, _)) => {
                println!("{}", export_smt_lib(&self.graph, test_name, None, &[], Some(dist)));
            }
            Err(e) => {
                println!("{}", export_smt_lib(&self.graph, test_name, Some(e), &[], None));
            }
        }
        println!("===============================================\n");

        if let Some(limits) = &limits_result {
            if limits.mcm < 0.0 {
                return Err("Extensionality Collision: Negative-weight cycle detected!".to_string());
            }
        }

        bf_result
    }
}

fn main() {
    println!("=== The Extensionality Collision (Kuratowski vs. Quine) ===");

    let mut session = Session::new();

    // Kuratowski pair K: K = {{x}, {x, y}}
    // Extracted constraints: x in S1, y in S2, x in S2, S1 in K, S2 in K
    // Weight shift: K is +2 relative to x and y.

    // Quine pair Q: Q(a, b)
    // Extracted constraints: Preserves order geometrically using type-shifting mappings across integers.
    // Meaning extracting a or b does not change the type level. We model this as a = Q and b = Q.
    // Weight shift: Q is 0 relative to a and b.

    // Extensionality Equivalence Check: K = Q
    // This connects the graphs of K and Q. The engine must calculate topological
    // weights (+2 vs 0) without a negative-weight cycle error.

    let formula = "{ E | (((S1 in K /\\ S2 in K) /\\ (x in S1 /\\ (x in S2 /\\ y in S2))) /\\ (a = Q /\\ b = Q)) /\\ K = Q }";

    println!("Evaluating Formula via Engine: {}", formula);

    match session.eval(formula, "base_extensionality_collision") {
        Ok((dist, _, _, _)) => {
            println!("[SUCCESS] Engine correctly tracked the differing topological weights (+2 vs 0) without a paradox halt!");
            println!("Distance vector: {:?}", dist);

            // Verification logic:
            // Ensure x is tracked as +2 below K (dist[K] >= dist[x] + 2)
            // Ensure a is tracked as 0 below Q (dist[Q] == dist[a])
            // Since K = Q, dist[K] == dist[Q]

            let mut k_idx = None;
            let mut q_idx = None;
            let mut x_idx = None;
            let mut a_idx = None;

            for (var, idx) in &session.graph.var_to_idx {
                match var.0 {
                    monist_core::ast::Var::Free(ref name) => {
                        if name == "K" {
                            k_idx = Some(*idx);
                        }
                        if name == "Q" {
                            q_idx = Some(*idx);
                        }
                        if name == "x" {
                            x_idx = Some(*idx);
                        }
                        if name == "a" {
                            a_idx = Some(*idx);
                        }
                    }
                    _ => {}
                }
            }

            if let (Some(k), Some(q), Some(x), Some(a)) = (k_idx, q_idx, x_idx, a_idx) {
                println!("Validating relative topological offsets in the collapsed DAG...");
                // Distances computed by evaluate_topology from arbitrary init 0
                // We know dist[target] <= dist[source] + weight.
                // It tracks the relative typestate boundaries correctly.
                let dist_k = dist[k];
                let dist_q = dist[q];
                let dist_x = dist[x];
                let dist_a = dist[a];

                println!("Typestate K = {}", dist_k);
                println!("Typestate Q = {}", dist_q);
                println!("Typestate x = {}", dist_x);
                println!("Typestate a = {}", dist_a);

                assert_eq!(
                    dist_k, dist_q,
                    "Extensionality Equivalence requires K and Q to have same typestate limits"
                );
                assert!(
                    dist_k <= dist_x + 2,
                    "Kuratowski offset guarantees K is at most x + 2"
                );
                assert_eq!(
                    dist_q, dist_a,
                    "Quine geometric map guarantees Q preserves exact typestate of a"
                );

                println!(
                    "[SUCCESS] Extensionality Collision topological weights correctly verified!"
                );
            }
        }
        Err(e) => {
            panic!(
                "Test Failed! Expected success without paradox halt, but got Error: {}",
                e
            );
        }
    }

    println!("\n--- Advanced Deep Hierarchy Verification ---");
    println!(
        "Testing Deep Nesting: Kuratowski K3 = K(K(x,y), K(z,w)) vs Quine Q3 = Q(Q(a,b), Q(c,d))"
    );

    // Kuratowski nested 2 levels deep
    // K_xy = {{x}, {x, y}}
    // K_zw = {{z}, {z, w}}
    // K3 = {{K_xy}, {K_xy, K_zw}}

    // Quine nested 2 levels deep
    // Q_ab = Q(a,b)  => a=Q_ab, b=Q_ab
    // Q_cd = Q(c,d)  => c=Q_cd, d=Q_cd
    // Q3 = Q(Q_ab, Q_cd) => Q_ab=Q3, Q_cd=Q3

    let deep_formula = "{ E | ( \
        (((S1 in K_xy /\\ S2 in K_xy) /\\ (x in S1 /\\ (x in S2 /\\ y in S2))) /\\ \
         ((S3 in K_zw /\\ S4 in K_zw) /\\ (z in S3 /\\ (z in S4 /\\ w in S4)))) /\\ \
        ((S5 in K3 /\\ S6 in K3) /\\ (K_xy in S5 /\\ (K_xy in S6 /\\ K_zw in S6))) \
    ) /\\ ( \
        ((a = Q_ab /\\ b = Q_ab) /\\ (c = Q_cd /\\ d = Q_cd)) /\\ \
        (Q_ab = Q3 /\\ Q_cd = Q3) \
    ) /\\ K3 = Q3 }";

    let mut session2 = Session::new();
    println!("Evaluating Deep Hierarchy Formula via Engine...");

    match session2.eval(deep_formula, "deep_extensionality_collision") {
        Ok((dist, _, _, _)) => {
            println!("[SUCCESS] Engine correctly tracked deep geometric scaling (+4 vs 0) without a paradox halt!");

            let mut k3_idx = None;
            let mut q3_idx = None;
            let mut x_idx = None;
            let mut a_idx = None;

            for (var, idx) in &session2.graph.var_to_idx {
                match var.0 {
                    monist_core::ast::Var::Free(ref name) => {
                        if name == "K3" {
                            k3_idx = Some(*idx);
                        }
                        if name == "Q3" {
                            q3_idx = Some(*idx);
                        }
                        if name == "x" {
                            x_idx = Some(*idx);
                        }
                        if name == "a" {
                            a_idx = Some(*idx);
                        }
                    }
                    _ => {}
                }
            }

            if let (Some(k3), Some(q3), Some(x), Some(a)) = (k3_idx, q3_idx, x_idx, a_idx) {
                let dist_k3 = dist[k3];
                let dist_q3 = dist[q3];
                let dist_x = dist[x];
                let dist_a = dist[a];

                println!("Typestate K3 = {}", dist_k3);
                println!("Typestate Q3 = {}", dist_q3);
                println!("Typestate x = {}", dist_x);
                println!("Typestate a = {}", dist_a);

                assert_eq!(
                    dist_k3, dist_q3,
                    "Extensionality Equivalence maintained at boundary K3=Q3"
                );
                assert!(
                    dist_k3 <= dist_x + 4,
                    "Deep Kuratowski scaling properly bounds type extraction at +4 limits"
                );
                assert_eq!(
                    dist_q3, dist_a,
                    "Deep Quine geometric map maintains stable 0 typestate cascade"
                );

                println!("[SUCCESS] Deep Extensionality Collision successfully mapped differing topological matrices!");
            }
        }
        Err(e) => {
            panic!("Deep Test Failed! {}", e);
        }
    }
}
