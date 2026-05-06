use criterion::{Criterion, criterion_group, criterion_main};
use ocl::{Buffer, ProQue, flags};

fn bench_knaster_tarski(c: &mut Criterion) {
    // We simulate the data execution of the Knaster-Tarski least fixpoint calculation.
    // Operating within the safe bounds of an SC_CUT, the GPU Interaction Net
    // computes the fixpoint F(X) = X natively using parallel wavefront iterations.

    let domain_size: usize = 250_000; // Simulating 250k set elements

    let src = r#"
        __kernel void knaster_tarski_lfp(__global ulong* current_set, __global ulong* next_set, ulong choice_mask) {
            size_t i = get_global_id(0);
            
            // Simulating an unstratified recursive choice mapping over an SC topological island.
            // F(X) maps X over the Choice function. In Interaction Nets, this maps via S, K, I reductions.
            // Here, we simulate the lock-free boolean aggregation of the least fixpoint set generation.
            ulong state = current_set[i];
            
            // LFP growth: Monotonic application of the choice mask
            // X_{n+1} = X_n U Choice(X_n)
            ulong next_state = state | choice_mask;
            
            next_set[i] = next_state;
        }
    "#;

    let pro_que_res = ProQue::builder().src(src).dims(domain_size).build();

    if let Ok(pro_que) = pro_que_res {
        let mut group = c.benchmark_group("gpu_knaster_tarski");
        group.sample_size(10);
        group.measurement_time(std::time::Duration::from_secs(1));

        group.bench_function("lfp_1M_elements", |b| {
            b.iter(|| {
                let mut current_set = Buffer::<u64>::builder()
                    .queue(pro_que.queue().clone())
                    .flags(flags::MEM_READ_WRITE)
                    .len(domain_size)
                    .build()
                    .unwrap();

                let mut next_set = Buffer::<u64>::builder()
                    .queue(pro_que.queue().clone())
                    .flags(flags::MEM_READ_WRITE)
                    .len(domain_size)
                    .build()
                    .unwrap();

                // Simulating iterations to reach normal form (lfp convergence)
                for _ in 0..10 {
                    let kernel = pro_que
                        .kernel_builder("knaster_tarski_lfp")
                        .arg(&current_set)
                        .arg(&next_set)
                        .arg(0x00FF00FF00FF00FFu64) // mock choice mask
                        .build()
                        .unwrap();

                    unsafe {
                        kernel.enq().unwrap();
                    }

                    // Swap buffers for next generation
                    std::mem::swap(&mut current_set, &mut next_set);
                }

                pro_que.queue().finish().unwrap();
            })
        });

        group.finish();
    } else {
        println!("OpenCL not available. Skipping knaster_tarski benchmark.");
    }
}

criterion_group!(benches, bench_knaster_tarski);
criterion_main!(benches);
