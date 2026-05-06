use criterion::{Criterion, criterion_group, criterion_main};
use monist_macros::stage;
use ocl::{Buffer, ProQue, flags};

// We use the #[stage] macro output here nominally to mark it,
// though OpenCL requires string kernels for the build.
#[stage]
fn create_ocl_proque() -> ocl::Result<ProQue> {
    // A mock OpenCL kernel string representing basic node collisions.
    // We treat each u64 as a packed interaction combinator node.
    let src = r#"
        __kernel void node_collision(__global ulong* nodes, __global ulong* results) {
            size_t i = get_global_id(0);
            
            // Mock node collision logic:
            // Swap or update atomic state based on node rules
            ulong node = nodes[i];
            
            // Simulate combinator interaction (e.g. ann, era)
            // In a real system this uses atomic_cmpxchg on 64-bit graph memory
            ulong next_state = node ^ 0x0123456789ABCDEF;
            
            results[i] = next_state;
        }
    "#;

    ProQue::builder().src(src).dims(250).build()
}

fn bench_ocl_bridge(c: &mut Criterion) {
    // If there is no OpenCL device, we just skip or gracefully handle it.
    let pro_que_res = create_ocl_proque();

    if let Ok(pro_que) = pro_que_res {
        let mut group = c.benchmark_group("gpu_ocl_bridge");
        group.sample_size(10);
        group.measurement_time(std::time::Duration::from_secs(1));

        group.bench_function("node_collision_dispatch", |b| {
            b.iter(|| {
                let nodes_buffer = Buffer::<u64>::builder()
                    .queue(pro_que.queue().clone())
                    .flags(flags::MEM_READ_WRITE)
                    .len(250)
                    .build()
                    .unwrap();

                let results_buffer = Buffer::<u64>::builder()
                    .queue(pro_que.queue().clone())
                    .flags(flags::MEM_READ_WRITE)
                    .len(250)
                    .build()
                    .unwrap();

                let kernel = pro_que
                    .kernel_builder("node_collision")
                    .arg(&nodes_buffer)
                    .arg(&results_buffer)
                    .build()
                    .unwrap();

                unsafe {
                    kernel.enq().unwrap();
                }

                pro_que.queue().finish().unwrap();
            })
        });
        group.finish();
    } else {
        println!("OpenCL not available. Skipping ocl_bridge benchmark.");
    }
}

criterion_group!(benches, bench_ocl_bridge);
criterion_main!(benches);
