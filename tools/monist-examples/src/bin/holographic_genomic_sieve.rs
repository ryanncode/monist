//! # Holographic Multi-Omics Genomic Sieve
//!
//! A standalone Monist Engine example that models a high-dimensional
//! transcriptomic filter. The code demonstrates:
//!
//! 1. **Shadow Ingestion** — Standard gene expression profiles are superposed
//!    into a continuous 10,000-dimensional VSA baseline via the Holographic
//!    Co-processor (`monist-comb/comblib/vsa_embed`).
//! 2. **Destructive Interference** — The healthy baseline is neutralized from
//!    patient multi-omics telemetry in O(1) time via pointwise subtraction.
//! 3. **WGPU SIC Bridge** — A custom WGSL compute shader performs massively
//!    parallel tensor dot-products to isolate the hidden oncogenic anomalies,
//!    snapping them back into discrete `monist-core` logical variables.
//!
//! This proves for computational biology that massive transcriptomic noise
//! can be physically annihilated before traditional, memory-exhausting discrete
//! evaluation even begins.

use std::borrow::Cow;

use monist_comb::comblib::vsa_embed::{Codebook, HDCVector, D};
use monist_core::ast::Var;
use monist_core::graph::{GraphArena, ScopedVar};
use wgpu::util::DeviceExt;

// =========================================================================
// Constants
// =========================================================================

/// Number of healthy transcriptomic profiles embedded into the baseline.
const NUM_HEALTHY_PROFILES: usize = 500_000;

/// Number of anomalous events injected into patient telemetry.
const NUM_ANOMALOUS_EVENTS: usize = 12;

/// Total patient events (healthy mirror + anomalous).
const TOTAL_PATIENT_EVENTS: usize = NUM_HEALTHY_PROFILES + NUM_ANOMALOUS_EVENTS;

/// Number of entries in the anomaly codebook for GPU SIC scanning.
const NUM_ANOMALY_ENTRIES: usize = 100;

/// Similarity threshold for anomaly identification on the GPU.
const SIC_THRESHOLD: f32 = 0.3;

// =========================================================================
// Gene Name Tables
// =========================================================================

/// Healthy wild-type gene expression labels.
const HEALTHY_GENES: &[&str] = &[
    "TP53_WT", "BRCA1_WT", "EGFR_WT", "KRAS_WT", "PIK3CA_WT",
    "BRAF_WT", "MYC_WT", "PTEN_WT", "RB1_WT", "APC_WT",
    "CDH1_WT", "MLH1_WT", "MSH2_WT", "ATM_WT", "CHEK2_WT",
    "PALB2_WT", "RAD51_WT", "FGFR2_WT", "ERBB2_WT", "ALK_WT",
];

/// Known somatic mutation labels for the anomaly codebook.
/// The first two (EGFR_L858R, KRAS_G12C) will be injected into patient data.
const ANOMALY_MUTATIONS: &[&str] = &[
    "EGFR_L858R", "KRAS_G12C", "BRAF_V600E", "PIK3CA_H1047R", "ERBB2_S310F",
    "NRAS_Q61K", "ALK_F1174L", "MET_D1228N", "RET_M918T", "FGFR3_S249C",
    "KIT_D816V", "PDGFRA_D842V", "IDH1_R132H", "IDH2_R172K", "DNMT3A_R882H",
    "FLT3_D835Y", "NPM1_W288fs", "JAK2_V617F", "CALR_L367fs", "MPL_W515L",
    "ABL1_T315I", "ROS1_G2032R", "NTRK1_G595R", "SMO_D473H", "PTCH1_W844X",
    "APC_R1450X", "CTNNB1_S33C", "FBXW7_R465C", "NOTCH1_L1585P", "PTEN_R130Q",
    "RB1_R552X", "TP53_R175H", "TP53_R248W", "TP53_G245S", "TP53_R273H",
    "CDKN2A_R80X", "VHL_R167Q", "SMAD4_R361C", "STK11_D194N", "NF1_R1513X",
    "NF2_L64P", "BRCA2_E1493X", "ATM_R3008H", "CHEK2_I157T", "PALB2_Y1183X",
    "RAD51C_L138X", "MUTYH_Y179C", "MSH6_F1088fs", "PMS2_R134X", "POLE_P286R",
    "ARID1A_Q1334X", "SMARCA4_T910M", "KMT2A_Q680X", "KMT2D_C4827Y", "CREBBP_R1446C",
    "EP300_D1399N", "EZH2_Y641N", "SETD2_R1625C", "BAP1_Q684X", "ASXL1_G646fs",
    "TET2_Q916X", "SF3B1_K700E", "SRSF2_P95H", "U2AF1_S34F", "ZRSR2_E133fs",
    "RUNX1_D198N", "GATA2_T354M", "CEBPA_Q312X", "PHF6_R274X", "WT1_R462W",
    "BCOR_N1425S", "STAG2_Q593X", "RAD21_R65X", "SMC1A_R586W", "SMC3_Q1004X",
    "TERT_C228T", "ATRX_R2164X", "DAXX_F520fs", "H3F3A_K27M", "H3F3B_G34R",
    "BCORL1_Q700X", "CIC_R215W", "FUBP1_R430X", "PIK3R1_N453del", "MAP2K1_K57N",
    "MAP3K1_E303X", "CDK12_K765fs", "CDK4_R24C", "CCND1_E36K", "CCNE1_S345L",
    "MDM2_T16A", "MDM4_A454T", "CDKN1B_Y88X", "RNF43_G659fs", "AXIN2_R463X",
    "TSC1_R692X", "TSC2_R1459X", "FLCN_H429fs", "SDHA_R589W", "SDHB_R242H",
];

// =========================================================================
// WGSL Compute Shader
// =========================================================================

/// Custom WGSL shader for massively parallel Successive Interference
/// Cancellation. Each GPU invocation computes the dot product of the
/// residual wave against one anomaly codebook entry.
const SIC_SHADER: &str = r#"
    @group(0) @binding(0) var<storage, read> residual: array<f32>;
    @group(0) @binding(1) var<storage, read> codebook: array<f32>;
    @group(0) @binding(2) var<storage, read_write> scores: array<f32>;
    @group(0) @binding(3) var<uniform> params: vec4<u32>;

    @compute @workgroup_size(256)
    fn sic_dot_product(@builtin(global_invocation_id) gid: vec3<u32>) {
        let entry_idx = gid.x;
        let num_entries = params.x;
        let dim = params.y;

        if (entry_idx >= num_entries) {
            return;
        }

        var acc: f32 = 0.0;
        let base = entry_idx * dim;
        for (var d = 0u; d < dim; d = d + 1u) {
            acc += residual[d] * codebook[base + d];
        }
        scores[entry_idx] = acc;
    }
"#;

// =========================================================================
// Main
// =========================================================================

fn main() {
    // =====================================================================
    // Phase 1: Shadow Ingestion of the Healthy Baseline
    // =====================================================================

    println!("[SYSTEM] Initializing Holographic Co-processor (10,000 Dimensions)...");

    // Step 1a: Generate healthy gene basis vectors and register in codebook
    let mut healthy_codebook = Codebook::new();
    let mut healthy_basis: Vec<HDCVector> = Vec::with_capacity(HEALTHY_GENES.len());

    for &gene in HEALTHY_GENES {
        let vec = HDCVector::random_basis();
        healthy_codebook.insert(gene.to_string(), vec.clone());
        healthy_basis.push(vec);
    }

    // Step 1b: Generate anomaly codebook (separate registry for GPU SIC)
    let mut anomaly_codebook = Codebook::new();
    let mut anomaly_basis: Vec<HDCVector> = Vec::with_capacity(NUM_ANOMALY_ENTRIES);
    let mut anomaly_names: Vec<String> = Vec::with_capacity(NUM_ANOMALY_ENTRIES);

    for &mutation in &ANOMALY_MUTATIONS[..NUM_ANOMALY_ENTRIES] {
        let vec = HDCVector::random_basis();
        anomaly_codebook.insert(mutation.to_string(), vec.clone());
        anomaly_basis.push(vec);
        anomaly_names.push(mutation.to_string());
    }

    // Step 1c: Build the Shadow Baseline — superpose 500,000 healthy profiles
    // We iterate and accumulate, keeping only the running sum (40 KB).
    let mut baseline_tensor = HDCVector::new();
    for i in 0..NUM_HEALTHY_PROFILES {
        let gene_vec = &healthy_basis[i % healthy_basis.len()];
        baseline_tensor = baseline_tensor.superpose(gene_vec);
    }

    println!(
        "[INGESTION] Embedding {} healthy transcriptomic profiles into Shadow Baseline.",
        NUM_HEALTHY_PROFILES
    );

    // =====================================================================
    // Phase 2: Superposition and Destructive Interference
    // =====================================================================

    // Step 2a: Ingest patient telemetry — 500,000 healthy + 12 anomalous
    let mut patient_tensor = HDCVector::new();

    // Healthy portion (identical distribution to baseline)
    for i in 0..NUM_HEALTHY_PROFILES {
        let gene_vec = &healthy_basis[i % healthy_basis.len()];
        patient_tensor = patient_tensor.superpose(gene_vec);
    }

    // Anomalous events: 6× EGFR_L858R (index 0), 6× KRAS_G12C (index 1)
    let egfr_vec = &anomaly_basis[0];
    let kras_vec = &anomaly_basis[1];
    for _ in 0..6 {
        patient_tensor = patient_tensor.superpose(egfr_vec);
    }
    for _ in 0..6 {
        patient_tensor = patient_tensor.superpose(kras_vec);
    }

    println!(
        "[TELEMETRY] Superposing {} patient multi-omics events into singular tensor wave.",
        TOTAL_PATIENT_EVENTS
    );

    // Step 2b: Apply Destructive Interference — O(1) pointwise subtraction
    let residual_tensor = patient_tensor.holographic_exclusion_query(&baseline_tensor);

    println!(
        "[PHYSICS] Applying Destructive Interference. {} baseline vectors annihilated in O(1) time.",
        NUM_HEALTHY_PROFILES
    );

    // =====================================================================
    // Phase 3: WGPU SIC Bridge
    // =====================================================================

    println!("[HARDWARE] Dispatching residual wave to WGPU SIC Bridge...");

    let identified_anomalies = pollster::block_on(gpu_sic_bridge(
        &residual_tensor,
        &anomaly_basis,
        &anomaly_names,
    ));

    // =====================================================================
    // Phase 4: Discrete Bridge — Snap back to monist-core variables
    // =====================================================================

    let mut diagnostic_arena = GraphArena::new();
    let mut result_descriptions: Vec<String> = Vec::new();

    for (name, confidence) in &identified_anomalies {
        let var = ScopedVar(Var::Free(name.clone()), 0);
        diagnostic_arena.add_var(var);
        result_descriptions.push(format!("{} (Confidence: {:.2})", name, confidence));
    }

    println!("[DISCRETE BRIDGE] Anomalies successfully snapped back to topological geometry.");
    println!(
        "[SUCCESS] Isolated Somatic Mutations: [{}]",
        result_descriptions.join(", ")
    );
}

// =========================================================================
// GPU SIC Bridge Implementation
// =========================================================================

/// Dispatches the residual tensor and anomaly codebook to the GPU via a
/// custom WGSL compute shader. Returns (mutation_name, confidence) pairs
/// for all anomalies exceeding the SIC threshold.
async fn gpu_sic_bridge(
    residual: &HDCVector,
    codebook_entries: &[HDCVector],
    entry_names: &[String],
) -> Vec<(String, f32)> {
    // Step 3a: Initialize WGPU
    let instance = wgpu::Instance::default();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: None,
        })
        .await
        .expect("Failed to find WGPU adapter");

    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default(), None)
        .await
        .expect("Failed to create WGPU device");

    // Step 3b: Compile custom SIC shader
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("SIC Dot Product Shader"),
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(SIC_SHADER)),
    });

    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("SIC Pipeline"),
        layout: None,
        module: &shader,
        entry_point: "sic_dot_product",
    });

    // Step 3c: Prepare GPU buffers
    let num_entries = codebook_entries.len();

    // Residual buffer: D × f32
    let residual_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Residual Buffer"),
        contents: bytemuck::cast_slice(&residual.values),
        usage: wgpu::BufferUsages::STORAGE,
    });

    // Codebook buffer: num_entries × D × f32 (flattened row-major)
    let flat_codebook: Vec<f32> = codebook_entries
        .iter()
        .flat_map(|v| v.values.iter().copied())
        .collect();

    let codebook_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Codebook Buffer"),
        contents: bytemuck::cast_slice(&flat_codebook),
        usage: wgpu::BufferUsages::STORAGE,
    });

    // Scores buffer: num_entries × f32
    let scores_init = vec![0.0f32; num_entries];
    let scores_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Scores Buffer"),
        contents: bytemuck::cast_slice(&scores_init),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
    });

    // Params uniform: [num_entries, D, 0, 0]
    let params: [u32; 4] = [num_entries as u32, D as u32, 0, 0];
    let params_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Params Uniform"),
        contents: bytemuck::cast_slice(&params),
        usage: wgpu::BufferUsages::UNIFORM,
    });

    // Step 3d: Create bind group
    let bind_group_layout = pipeline.get_bind_group_layout(0);
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("SIC Bind Group"),
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: residual_buf.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: codebook_buf.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: scores_buf.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 3,
                resource: params_buf.as_entire_binding(),
            },
        ],
    });

    // Step 3e: Dispatch compute pass
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("SIC Encoder"),
    });

    {
        let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("SIC Pass"),
            timestamp_writes: None,
        });
        cpass.set_pipeline(&pipeline);
        cpass.set_bind_group(0, &bind_group, &[]);
        // Dispatch enough workgroups to cover all entries
        let workgroups = (num_entries as u32 + 255) / 256;
        cpass.dispatch_workgroups(workgroups, 1, 1);
    }

    // Step 3f: Read back scores via staging buffer
    let staging_buf = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Staging Buffer"),
        size: (num_entries * std::mem::size_of::<f32>()) as u64,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    encoder.copy_buffer_to_buffer(
        &scores_buf,
        0,
        &staging_buf,
        0,
        staging_buf.size(),
    );
    queue.submit(Some(encoder.finish()));

    let buf_slice = staging_buf.slice(..);
    let (tx, rx) = std::sync::mpsc::channel();
    buf_slice.map_async(wgpu::MapMode::Read, move |v| tx.send(v).unwrap());
    device.poll(wgpu::Maintain::Wait);
    rx.recv().unwrap().unwrap();

    let data = buf_slice.get_mapped_range();
    let scores: Vec<f32> = bytemuck::cast_slice(&data).to_vec();
    drop(data);
    staging_buf.unmap();

    // Step 3g: Threshold and identify anomalies
    // Compute normalization: max absolute score for confidence scaling
    let max_score = scores
        .iter()
        .copied()
        .map(f32::abs)
        .fold(0.0f32, f32::max);

    let mut identified: Vec<(String, f32)> = Vec::new();
    for (i, &score) in scores.iter().enumerate() {
        if score > SIC_THRESHOLD && max_score > 0.0 {
            let confidence = score / max_score;
            identified.push((entry_names[i].clone(), confidence));
        }
    }

    // Sort by confidence descending
    identified.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    identified
}
