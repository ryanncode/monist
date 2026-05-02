use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ocl::{ProQue, Buffer, flags};

fn bench_latbol_simulation(c: &mut Criterion) {
    // We implement an interaction-combinator variant of a lattice-Boltzmann
    // iterative simulation to explicitly measure lock-free atomic `swap` contention
    // and GPU wave alignment under maximum occupancy.

    let grid_size = 1024 * 1024; // 1M cells
    let iterations = 10; // For benchmarking, simulate 10 steps

    // To measure lock-free atomic swap contention natively in OpenCL, we use atomic_xchg.
    // NOTE: In OpenCL 1.2, atomics on 64-bit integers require cl_khr_int64_base_atomics.
    let src = r#"
        #pragma OPENCL EXTENSION cl_khr_int64_base_atomics : enable

        __kernel void latbol_step(__global ulong* current_grid, __global ulong* next_grid) {
            size_t id = get_global_id(0);
            size_t size = get_global_size(0);
            
            // Interaction Combinator Lattice-Boltzmann step:
            // Instead of floating-point densities, we route discrete combinator tokens (u64).
            
            // Gather from neighbors (1D simplified neighborhood for wave alignment test)
            size_t left = (id == 0) ? size - 1 : id - 1;
            size_t right = (id == size - 1) ? 0 : id + 1;
            
            // Simulate lock-free atomic contention by swapping tokens aggressively
            // In a real system, nodes swap ports to interact.
            ulong my_val = atom_xchg(&current_grid[id], 0);
            ulong left_val = atom_xchg(&current_grid[left], 0);
            ulong right_val = atom_xchg(&current_grid[right], 0);
            
            // Collision & Streaming pseudo-logic
            ulong new_val = my_val ^ (left_val << 1) ^ (right_val >> 1);
            
            next_grid[id] = new_val;
        }
    "#;

    let pro_que_res = ProQue::builder()
        .src(src)
        .dims(grid_size)
        .build();

    if let Ok(pro_que) = pro_que_res {
        let mut group = c.benchmark_group("gpu_latbol_simulation");
        
        group.bench_function("latbol_wave_alignment", |b| {
            b.iter(|| {
                let grid_a = Buffer::<u64>::builder()
                    .queue(pro_que.queue().clone())
                    .flags(flags::MEM_READ_WRITE)
                    .len(grid_size)
                    .build()
                    .unwrap();

                let grid_b = Buffer::<u64>::builder()
                    .queue(pro_que.queue().clone())
                    .flags(flags::MEM_READ_WRITE)
                    .len(grid_size)
                    .build()
                    .unwrap();

                for step in 0..iterations {
                    let (curr, next) = if step % 2 == 0 {
                        (&grid_a, &grid_b)
                    } else {
                        (&grid_b, &grid_a)
                    };
                    
                    let kernel = pro_que.kernel_builder("latbol_step")
                        .arg(curr)
                        .arg(next)
                        .build()
                        .unwrap();

                    unsafe {
                        kernel.enq().unwrap();
                    }
                }
                
                pro_que.queue().finish().unwrap();
            })
        });
        
        group.finish();
    } else {
        println!("OpenCL not available or cl_khr_int64_base_atomics not supported. Skipping latbol_simulation benchmark.");
    }
}

criterion_group!(benches, bench_latbol_simulation);
criterion_main!(benches);
