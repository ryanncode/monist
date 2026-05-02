use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use monist_core::graph::{GraphArena, ScopedVar};
use monist_core::ast::Var;

fn generate_entangled_cycle_graph(size: usize) -> GraphArena {
    let mut arena = GraphArena::new();
    
    // Create a large cycle of 0-weight edges
    for i in 0..size {
        let v1 = ScopedVar(Var::Free(format!("v{}", i)), 0);
        let v2 = ScopedVar(Var::Free(format!("v{}", (i + 1) % size)), 0);
        let u = arena.add_var(v1);
        let v = arena.add_var(v2);
        // Equality constraint -> bidirectional 0-weight edges
        arena.edges.push((u, v, 0));
        arena.edges.push((v, u, 0));
    }
    
    // Add some random cross edges to make it entangled
    for i in 0..(size * 100) {
        let v1 = ScopedVar(Var::Free(format!("v{}", i)), 0);
        let v2 = ScopedVar(Var::Free(format!("v{}", (i + size / 2) % size)), 0);
        let u = arena.add_var(v1);
        let v = arena.add_var(v2);
        arena.edges.push((u, v, 1));
    }

    arena
}

fn bench_acyclic_profile(c: &mut Criterion) {
    let mut group = c.benchmark_group("Acyclic Profile");
    group.sample_size(10);
    group.warm_up_time(std::time::Duration::from_millis(500));
    group.measurement_time(std::time::Duration::from_secs(1));
    
    for size in [100, 1000, 5000, 10000].iter() {
        group.bench_with_input(BenchmarkId::new("collapse_scc", size), size, |b, &size| {
            b.iter(|| {
                let mut graph = generate_entangled_cycle_graph(size);
                graph.collapse_scc_0_weight();
                // Optionally run bellman_ford
                let _ = graph.bellman_ford();
            });
        });
    }
    
    group.finish();
}

criterion_group!(benches, bench_acyclic_profile);
criterion_main!(benches);
