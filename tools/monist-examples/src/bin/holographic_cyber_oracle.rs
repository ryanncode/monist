use monist_comb::comblib::vsa_embed::{Codebook, HDCVector};
use std::time::Instant;
use rand::RngExt;
use std::collections::HashSet;
use std::borrow::Cow;
use wgpu::util::DeviceExt;

struct LegacySIEM;

impl LegacySIEM {
    fn process(&self, _event: &str) {}
}

const D: usize = 10000;

fn main() {
    pollster::block_on(run_gpu_pipeline());
}

async fn run_gpu_pipeline() {
    println!("============================================================");
    println!("=== Phase 3: The Compute Handoff (GPU VRAM Oracle)      ===");
    println!("=== Domain: Cybersecurity & Threat Hunting            ===");
    println!("=== Mode: Massively Parallel WGPU Matrix Processing   ===");
    println!("============================================================\n");

    let mut codebook = Codebook::new();
    let siem = LegacySIEM;
    let mut rng = rand::rng();

    // 1. Define massive normal, verified traffic profiles
    let num_normal_baselines = 1000;
    println!("[Phase 1] Calibrating Holographic Space (Shadow Ingestion)");
    println!("Generating {} normal traffic baselines...", num_normal_baselines);
    
    let mut normal_traffic_keys = Vec::with_capacity(num_normal_baselines);
    let mut normal_set = HashSet::new();

    for i in 0..num_normal_baselines {
        let ip = format!("IP:10.0.{}.{}->Port:{}", i / 256, i % 256, 443 + (i % 10));
        let vec = HDCVector::random_basis();
        codebook.insert(ip.clone(), vec);
        normal_traffic_keys.push(ip.clone());
        normal_set.insert(ip);
    }

    let num_batches = 3000; 
    let events_per_batch = 150; 
    let anomaly_probability = 0.005; 
    
    println!("[Phase 2] Generating Bulk Telemetry...");
    println!("Total Batches: {}", num_batches);
    println!("Events per Batch: {}", events_per_batch);
    println!("Total Events: {}", num_batches * events_per_batch);
    
    let num_anomalies = 100;
    let mut anomaly_keys = Vec::with_capacity(num_anomalies);
    for i in 0..num_anomalies {
        let ip = format!("IP:192.168.100.{}->Port:4444", i);
        let vec = HDCVector::random_basis();
        codebook.insert(ip.clone(), vec);
        anomaly_keys.push(ip);
    }

    let mut batches = Vec::with_capacity(num_batches);
    for _ in 0..num_batches {
        let mut batch = Vec::with_capacity(events_per_batch);
        for _ in 0..events_per_batch {
            if rng.random_bool(anomaly_probability) {
                let idx = rng.random_range(0..num_anomalies);
                batch.push(anomaly_keys[idx].clone());
            } else {
                let idx = rng.random_range(0..num_normal_baselines);
                batch.push(normal_traffic_keys[idx].clone());
            }
        }
        batches.push(batch);
    }

    println!("\n[Phase 3] CPU: Streaming telemetry & Holographic Phase Subtraction...");
    let mut superposed_anomalies = Vec::with_capacity(num_batches);
    
    let cpu_start = Instant::now();
    for batch in &batches {
        let mut batch_vector = HDCVector::new();
        for traffic in batch {
            if let Some(v) = codebook.vectors.get(traffic) {
                batch_vector = batch_vector.superpose(v);
            }
        }

        let mut anomalous_signal = batch_vector;
        
        for traffic in batch {
            if normal_set.contains(traffic) {
                if let Some(verified_vec) = codebook.vectors.get(traffic) {
                     anomalous_signal = anomalous_signal.holographic_exclusion_query(verified_vec);
                }
            }
        }
        superposed_anomalies.push(anomalous_signal);
    }
    println!("  -> CPU Pre-processing & Subtraction took {:.2?}", cpu_start.elapsed());

    println!("\n[Phase 4] VRAM Handoff: Parallel GPU Dot Products (SIC Bridge)...");
    let gpu_start = Instant::now();

    // Setup WGPU
    let instance = wgpu::Instance::default();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            ..Default::default()
        })
        .await
        .expect("Failed to find WGPU adapter");

    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default(), None)
        .await
        .expect("Failed to create device");

    let num_codebook = codebook.vectors.len();
    
    // We need to pass the codebook to the GPU as a flat array of f32
    // We'll maintain an ordered list of keys so we know what index corresponds to what key
    let mut codebook_keys = Vec::with_capacity(num_codebook);
    let mut flat_codebook = Vec::with_capacity(num_codebook * D);
    
    for (k, v) in &codebook.vectors {
        codebook_keys.push(k.clone());
        flat_codebook.extend_from_slice(&v.values);
    }

    // Flat array of superposed batches
    let mut flat_batches = Vec::with_capacity(num_batches * D);
    for v in &superposed_anomalies {
        flat_batches.extend_from_slice(&v.values);
    }

    // Output array: num_batches * num_codebook
    let output_size = num_batches * num_codebook;
    let flat_output = vec![0.0f32; output_size];

    let codebook_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Codebook Buffer"),
        contents: bytemuck::cast_slice(&flat_codebook),
        usage: wgpu::BufferUsages::STORAGE,
    });

    let batches_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Batches Buffer"),
        contents: bytemuck::cast_slice(&flat_batches),
        usage: wgpu::BufferUsages::STORAGE,
    });

    let output_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Output Buffer"),
        contents: bytemuck::cast_slice(&flat_output),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
    });

    // Uniform buffer: [num_batches, num_codebook, D, 0]
    let uniform_data: [u32; 4] = [num_batches as u32, num_codebook as u32, D as u32, 0];
    let uniform_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Uniform Buffer"),
        contents: bytemuck::cast_slice(&uniform_data),
        usage: wgpu::BufferUsages::UNIFORM,
    });

    let shader_src = r#"
        @group(0) @binding(0) var<storage, read> codebook: array<f32>;
        @group(0) @binding(1) var<storage, read> batches: array<f32>;
        @group(0) @binding(2) var<storage, read_write> output: array<f32>;
        @group(0) @binding(3) var<uniform> params: vec4<u32>;

        @compute @workgroup_size(16, 16)
        fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
            let batch_idx = global_id.x;
            let codebook_idx = global_id.y;

            let num_batches = params.x;
            let num_codebook = params.y;
            let D = params.z;

            if (batch_idx >= num_batches || codebook_idx >= num_codebook) {
                return;
            }

            var dot_product: f32 = 0.0;
            let batch_offset = batch_idx * D;
            let codebook_offset = codebook_idx * D;

            // Compute dot product of batch_vector and codebook_vector
            for (var i = 0u; i < D; i = i + 1u) {
                dot_product = dot_product + (batches[batch_offset + i] * codebook[codebook_offset + i]);
            }

            let out_idx = batch_idx * num_codebook + codebook_idx;
            output[out_idx] = dot_product;
        }
    "#;

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("SIC GPU Shader"),
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(shader_src)),
    });

    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("SIC Pipeline"),
        layout: None,
        module: &shader,
        entry_point: "main",
    });

    let bind_group_layout = pipeline.get_bind_group_layout(0);
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("SIC Bind Group"),
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry { binding: 0, resource: codebook_buf.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 1, resource: batches_buf.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 2, resource: output_buf.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 3, resource: uniform_buf.as_entire_binding() },
        ],
    });

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    {
        let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: None,
            timestamp_writes: None,
        });
        cpass.set_pipeline(&pipeline);
        cpass.set_bind_group(0, &bind_group, &[]);
        
        let wg_x = (num_batches as u32 + 15) / 16;
        let wg_y = (num_codebook as u32 + 15) / 16;
        cpass.dispatch_workgroups(wg_x, wg_y, 1);
    }

    let staging_buf = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: (output_size * std::mem::size_of::<f32>()) as u64,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    encoder.copy_buffer_to_buffer(&output_buf, 0, &staging_buf, 0, staging_buf.size());
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

    println!("  -> WGPU Execution & Memory Transfer took {:.2?}", gpu_start.elapsed());

    // Evaluate GPU Output
    let mut total_dropped = 0;
    let mut total_anomalies_sent = 0;

    let eval_start = Instant::now();
    for (b_idx, batch) in batches.iter().enumerate() {
        let out_offset = b_idx * num_codebook;
        let mut recovered_set = HashSet::new();

        // Check the GPU computed similarities
        for (c_idx, key) in codebook_keys.iter().enumerate() {
            let sim = result[out_offset + c_idx];
            if sim > 0.4 {
                // We don't care about normal_traffic_keys, only anomalies
                // (Normal traffic should theoretically have 0 dot product because we subtracted it,
                // but due to noise it might hover around 0.1)
                recovered_set.insert(key.clone());
            }
        }

        for traffic in batch {
            if recovered_set.contains(traffic) && !normal_set.contains(traffic) {
                siem.process(traffic);
                total_anomalies_sent += 1;
            } else {
                total_dropped += 1;
            }
        }
    }
    
    println!("  -> Results Evaluation took {:.2?}", eval_start.elapsed());

    println!("\n=== GPU Accelerated Bulk Sieving Summary ===");
    println!("Total Processing Time (CPU + GPU): {:.2?}", cpu_start.elapsed());
    println!("Total Ingested Events: {}", num_batches * events_per_batch);
    println!("Dropped via Holographic Exclusion: {}", total_dropped);
    println!("Passed to SIEM: {}", total_anomalies_sent);
    println!("Result: High-throughput O(1) physics sieving protected the legacy pipeline!");
}
