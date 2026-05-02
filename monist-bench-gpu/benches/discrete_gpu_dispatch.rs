use criterion::{criterion_group, criterion_main, Criterion};
use ocl::{Platform, Device, Buffer, flags, SpatialDims, ProQue};

fn bench_discrete_gpu_dispatch(c: &mut Criterion) {
    // Force RusticCL to expose discrete GPUs like AMD Radeon (radeonsi) or Intel (iris)
    // if not already set by the user environment.
    if std::env::var("RUSTICL_ENABLE").is_err() {
        unsafe {
            std::env::set_var("RUSTICL_ENABLE", "radeonsi,iris,nouveau");
        }
    }

    // 1. Enumerate and actively filter discrete GPU / NPU, rejecting CPU.
    let mut selected_device: Option<Device> = None;

    let platforms = Platform::list();
    for platform in platforms {
        // Find GPU devices
        if let Ok(devices) = Device::list(platform, Some(ocl::core::DeviceType::GPU)) {
            for device in devices {
                // Reject CPU explicitly
                if let Ok(name) = device.name() {
                    if !name.to_lowercase().contains("cpu") && !name.to_lowercase().contains("pocl") && !name.to_lowercase().contains("llvmpipe") {
                        selected_device = Some(device);
                        break;
                    }
                }
            }
        }
        if selected_device.is_some() {
            break;
        }
    }

    if let Some(device) = selected_device {
        println!("Selected Discrete GPU/NPU: {}", device.name().unwrap_or_default());

        let grid_size = 1024 * 1024 * 4; // 4M cells to stress GPU further
        let iterations = 20;

        let src = r#"
            #pragma OPENCL EXTENSION cl_khr_int64_base_atomics : enable

            __kernel void topology_reduction(__global ulong* current_grid, __global ulong* next_grid) {
                size_t id = get_global_id(0);
                size_t size = get_global_size(0);
                
                size_t left = (id == 0) ? size - 1 : id - 1;
                size_t right = (id == size - 1) ? 0 : id + 1;
                
                // Massive lock-free atomic topology reduction
                ulong my_val = atom_xchg(&current_grid[id], 0);
                ulong left_val = atom_xchg(&current_grid[left], 0);
                ulong right_val = atom_xchg(&current_grid[right], 0);
                
                ulong new_val = my_val ^ (left_val << 1) ^ (right_val >> 1);
                
                next_grid[id] = new_val;
            }
        "#;

        let pro_que_res = ProQue::builder()
            .src(src)
            .device(device)
            .dims(grid_size)
            .build();

        if let Ok(pro_que) = pro_que_res {
            let mut group = c.benchmark_group("discrete_gpu_dispatch");
            group.sample_size(50);
            group.measurement_time(std::time::Duration::from_secs(15));
            
            group.bench_function("topology_reduction", |b| {
                b.iter(|| {
                    let mut grid_a = Buffer::<u64>::builder()
                        .queue(pro_que.queue().clone())
                        .flags(flags::MEM_READ_WRITE)
                        .len(grid_size)
                        .build()
                        .unwrap();

                    let mut grid_b = Buffer::<u64>::builder()
                        .queue(pro_que.queue().clone())
                        .flags(flags::MEM_READ_WRITE)
                        .len(grid_size)
                        .build()
                        .unwrap();

                    for step in 0..iterations {
                        let (curr, next) = if step % 2 == 0 {
                            (&mut grid_a, &mut grid_b)
                        } else {
                            (&mut grid_b, &mut grid_a)
                        };
                        
                        let kernel = pro_que.kernel_builder("topology_reduction")
                            .global_work_size(SpatialDims::One(grid_size))
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
            println!("Failed to compile OpenCL kernel on the selected device. Possibly missing cl_khr_int64_base_atomics.");
        }
    } else {
        println!("No suitable discrete GPU/NPU found. Skipping discrete_gpu_dispatch benchmark.");
    }
}

criterion_group!(benches, bench_discrete_gpu_dispatch);
criterion_main!(benches);
