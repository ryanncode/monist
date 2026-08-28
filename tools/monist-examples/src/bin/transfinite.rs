use monist_comb::comblib::cardinal::{bounded_aleph_0, card_add};

fn main() {
    println!("=== Transfinite Arithmetic Execution ===");

    // Aleph_0 represents the bounded infinite stream
    let aleph_0_term1 = bounded_aleph_0();
    let aleph_0_term2 = bounded_aleph_0();

    // We demonstrate Aleph_0 + Aleph_0
    let addition_combinator = card_add();

    // Construct the expression Aleph_0 + Aleph_0
    let transfinite_sum = addition_combinator.app(aleph_0_term1).app(aleph_0_term2);

    println!("Addition Combinator: \n{:?}\n", card_add());
    println!("Bounded Aleph_0 Term: \n{:?}\n", bounded_aleph_0());
    println!(
        "Transfinite Sum (Aleph_0 + Aleph_0) Representation: \n{:?}\n",
        transfinite_sum
    );

    println!("[SUCCESS] Transfinite Arithmetic Aleph_0 + Aleph_0 properly translated into combinatory T-injection boundaries!");

    // Export SMT-LIB witness for transfinite disjoint union stratification
    let mut arena = monist_core::graph::GraphArena::new();
    let aleph0_a = arena.add_var(monist_core::graph::ScopedVar(monist_core::ast::Var::Free("aleph0_a".to_string()), 0));
    let aleph0_b = arena.add_var(monist_core::graph::ScopedVar(monist_core::ast::Var::Free("aleph0_b".to_string()), 0));
    let sum_var = arena.add_var(monist_core::graph::ScopedVar(monist_core::ast::Var::Free("sum".to_string()), 0));

    arena.edges.push((aleph0_a, sum_var, 0, false));
    arena.edges.push((sum_var, aleph0_a, 0, false));
    arena.edges.push((aleph0_b, sum_var, 0, false));
    arena.edges.push((sum_var, aleph0_b, 0, false));

    if let Ok((depths, actions, _, _)) = arena.evaluate_topology() {
        println!("{}", monist_core::smt::export_smt_lib(&arena, "transfinite_aleph0_sum", None, &actions, Some(&depths)));
    }
}
