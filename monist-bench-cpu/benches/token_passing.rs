use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use monist_comb::ir::Comb;

// Mimics a sequential token
pub struct Token(u64);

impl Token {
    pub fn new() -> Self {
        Token(0)
    }

    pub fn pass_with_effect<F>(&mut self, f: F)
    where
        F: FnOnce(),
    {
        // Execute the side effect
        f();
        // Advance token state
        self.0 = self.0.wrapping_add(1);
    }
    
    pub fn get_state(&self) -> u64 {
        self.0
    }
}

// Generates a mock pure reduction (e.g., deeply nested AST)
fn generate_pure_reductions(depth: usize) -> Comb {
    let mut comb = Comb::Var("x".to_string());
    for _ in 0..depth {
        comb = Comb::S.app(Comb::K).app(comb);
    }
    comb
}

fn bench_token_passing(c: &mut Criterion) {
    let mut group = c.benchmark_group("Token-Passing Throughput");
    group.sample_size(10);
    group.warm_up_time(std::time::Duration::from_millis(500));
    group.measurement_time(std::time::Duration::from_secs(1));
    
    for size in [100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::new("intertwined_io", size), size, |b, &size| {
            b.iter(|| {
                let mut token = Token::new();
                let mut side_effect_counter = 0;
                
                for _ in 0..size {
                    // 1. Sequential side-effect (guarded by token)
                    token.pass_with_effect(|| {
                        side_effect_counter += 1;
                        black_box(side_effect_counter);
                    });
                    
                    // 2. Pure interaction net reduction (mocked by AST traversal/abstraction)
                    let expr = generate_pure_reductions(10);
                    let abstracted = expr.abstract_var("x");
                    black_box(abstracted);
                }
                
                black_box(token.get_state());
            });
        });
    }
    
    group.finish();
}

criterion_group!(benches, bench_token_passing);
criterion_main!(benches);
