use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use monist_core::ast::Var;
use monist_core::eval::ExecutionLimits;
use monist_core::graph::{GraphArena, ScopedVar};

fn generate_non_well_founded_graph(size: usize) -> GraphArena {
    let mut arena = GraphArena::new();

    // Create a path of `size` nodes, and a back edge to simulate V in V but expanded.
    // V \in V means weight = -1.
    // We create a cycle of length `size` where the total weight is negative.

    for i in 0..size {
        let u = arena.add_var(ScopedVar(Var::Free(format!("v{}", i)), 0));
        let v = arena.add_var(ScopedVar(Var::Free(format!("v{}", (i + 1) % size)), 0));

        // Let's make every edge weight 0, except the last one which is -1.
        let weight = if i == size - 1 { -1 } else { 0 };
        arena.edges.push((u, v, weight, false));
    }

    arena
}

fn bench_k_iteration(c: &mut Criterion) {
    let mut group = c.benchmark_group("K-Iteration Exhaustion");
    group.sample_size(10);
    group.warm_up_time(std::time::Duration::from_millis(500));
    group.measurement_time(std::time::Duration::from_secs(1));

    for size in [100, 200, 500].iter() {
        group.bench_with_input(
            BenchmarkId::new("execution_limits", size),
            size,
            |b, &size| {
                b.iter(|| {
                    let graph = generate_non_well_founded_graph(size);
                    let _limits = ExecutionLimits::compute_for_graph(&graph);
                });
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_k_iteration);
criterion_main!(benches);
