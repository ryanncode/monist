use std::borrow::Cow;
use std::time::Instant;
use wgpu::util::DeviceExt;
use rand_distr::{Distribution, Normal};

pub const D: usize = 10000;
pub const NUM_VECTORS: usize = 1000; // 1000 vectors
pub const CYCLES: usize = 100; // 100 continuous GPU interaction cycles

#[derive(Clone)]
struct GpuVector {
    data: Vec<f32>,
}

fn main() {
    println!("=== Deep GPU Holographic Database Stress Test ===");
    println!("Initializing {} hypervectors of dimension {} on the CPU...", NUM_VECTORS, D);
    
    let mut rng = rand::rng();
    let normal = Normal::new(0.0, 1.0 / (D as f32).sqrt()).unwrap();

    let mut input_vectors = vec![GpuVector { data: vec![0.0; D] }; NUM_VECTORS];
    for vec in input_vectors.iter_mut() {
        for val in vec.data.iter_mut() {
            *val = normal.sample(&mut rng);
        }
    }
    
    println!("Spinning up WGPU to compute {} massive cyclic convolutions and phase interference loops...", CYCLES);
    pollster::block_on(run_gpu_vsa(input_vectors));
}

async fn run_gpu_vsa(input_vectors: Vec<GpuVector>) {
    let instance = wgpu::Instance::default();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            ..Default::default()
        })
        .await
        .expect("Failed to find WGPU adapter");

    println!("Detected Hardware Backend: {:?}", adapter.get_info().name);
    println!("Backend Type: {:?}", adapter.get_info().backend);
    println!("Device Type: {:?}", adapter.get_info().device_type);

    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default(), None)
        .await
        .expect("Failed to create device");

    let shader_src = r#"
        @group(0) @binding(0) var<storage, read_write> swarm_data: array<f32>;
        @group(0) @binding(1) var<uniform> cycle_params: vec4<u32>;

        @compute @workgroup_size(256)
        fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
            let dim_idx = global_id.x;
            if (dim_idx >= 10000u) {
                return;
            }

            let num_vectors = cycle_params.x;

            var local_accum: f32 = 0.0;
            
            for (var v = 0u; v < num_vectors; v = v + 1u) {
                let base_idx = v * 10000u;
                
                // Simulate an O(N^2) circular convolution-like interference pattern
                // We use a window of 1000 offsets.
                for (var window = 0u; window < 1000u; window = window + 1u) {
                    let target_idx = (dim_idx + window) % 10000u;
                    let val = swarm_data[base_idx + target_idx];
                    
                    if (window % 2u == 0u) {
                        local_accum = local_accum + val;
                    } else {
                        local_accum = local_accum - val;
                    }
                }
            }
            
            // Memory write barrier (Store the chaotic superposition back)
            swarm_data[dim_idx] = local_accum * 0.001;
        }
    "#;

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("VSA Taxing Shader"),
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(shader_src)),
    });

    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("VSA Pipeline"),
        layout: None,
        module: &shader,
        entry_point: "main",
    });

    let flat_input: Vec<f32> = input_vectors.iter().flat_map(|v| v.data.iter().copied()).collect();

    let input_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Swarm Vectors Buffer"),
        contents: bytemuck::cast_slice(&flat_input),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST,
    });

    let uniform_data: [u32; 4] = [NUM_VECTORS as u32, 0, 0, 0];
    let uniform_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Uniform Buffer"),
        contents: bytemuck::cast_slice(&uniform_data),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let bind_group_layout = pipeline.get_bind_group_layout(0);
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("VSA Bind Group"),
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: input_buf.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: uniform_buf.as_entire_binding(),
            },
        ],
    });

    println!("Executing {} continuous cycles of holographic binding across the GPU...", CYCLES);
    let start_time = Instant::now();

    for cycle in 1..=CYCLES {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: None,
                timestamp_writes: None,
            });
            cpass.set_pipeline(&pipeline);
            cpass.set_bind_group(0, &bind_group, &[]);
            
            let workgroups = (D as u32 + 255) / 256;
            cpass.dispatch_workgroups(workgroups, 1, 1);
        }
        
        queue.submit(Some(encoder.finish()));
        
        if cycle % 10 == 0 {
            println!("   -> Queued Cycle {}/{}...", cycle, CYCLES);
        }
    }

    // Force wait for all queued computations
    device.poll(wgpu::Maintain::Wait);

    let staging_buf = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: (D * std::mem::size_of::<f32>()) as u64,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    encoder.copy_buffer_to_buffer(&input_buf, 0, &staging_buf, 0, staging_buf.size());
    queue.submit(Some(encoder.finish()));

    let buf_slice = staging_buf.slice(..);
    let (tx, rx) = std::sync::mpsc::channel();
    buf_slice.map_async(wgpu::MapMode::Read, move |v| tx.send(v).unwrap());
    device.poll(wgpu::Maintain::Wait);
    rx.recv().unwrap().unwrap();

    let data = buf_slice.get_mapped_range();
    let result: Vec<f32> = bytemuck::cast_slice(&data).to_vec();
    drop(data);
    staging_buf.unmap();

    let elapsed = start_time.elapsed();
    println!("=== GPU Compute Complete ===");
    println!("Time Elapsed for Deep Execution: {:.2} seconds", elapsed.as_secs_f64());
    println!("Superposed VSA Tensor Output (first 5 dimensions): {:?}", &result[0..5]);
    println!("\n[Success] Sustained holographic matrix limits pushed across {} massive compute cycles.", CYCLES);
}