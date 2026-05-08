use crate::ast::GNet;
use bytemuck::{Pod, Zeroable};
use std::borrow::Cow;
use wgpu::util::DeviceExt;

/// Tracks the execution state of the WGPU compute pipeline.
/// This struct is aligned to 16 bytes for compatibility with WebGPU `struct` standards.
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct GpuState {
    pub active_nodes: u32,
    pub interactions: u32,
    pub free_list_head: u32,
    pub _padding: u32, // to align to 16 bytes
}

/// The core Physics Engine backend that drives Interaction Net graph reduction on the GPU.
/// It wraps `wgpu::Device` and `wgpu::Queue`, pre-compiling the `reduce.wgsl` shader
/// to perform massively parallel lock-free atomic CAS loops for Interaction Net rewrites.
pub struct WgpuExecutor {
    device: wgpu::Device,
    queue: wgpu::Queue,
    pipeline: wgpu::ComputePipeline,
    gc_pipeline: wgpu::ComputePipeline,
}

impl Default for WgpuExecutor {
    fn default() -> Self {
        Self::new()
    }
}

impl WgpuExecutor {
    pub fn new() -> Self {
        pollster::block_on(async {
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
                .expect("Failed to create device");

            let shader_src = include_str!("reduce.wgsl");
            let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Reduce Shader"),
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(shader_src)),
            });

            let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some("Reduce Pipeline"),
                layout: None,
                module: &shader,
                entry_point: "main",
            });

            let gc_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some("GC Pipeline"),
                layout: None,
                module: &shader,
                entry_point: "cycle_gc",
            });

            Self {
                device,
                queue,
                pipeline,
                gc_pipeline,
            }
        })
    }

    /// Executes a Graph Reduction pass on the provided `GNet` using WGPU compute shaders.
    /// Returns the updated `GNet` (downloaded from GPU memory) along with its final `GpuState`.
    /// 
    /// This method manages staging buffers and invokes Cycle Garbage Collection
    /// at the end of the physics evaluation.
    pub fn execute(&self, net: &GNet) -> (GNet, GpuState) {
        pollster::block_on(async {
            let arena_buf = self
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Arena Buffer"),
                    contents: bytemuck::cast_slice(&net.nodes),
                    usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
                });

            let free_list_buf = self
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Free List Buffer"),
                    contents: bytemuck::cast_slice(&net.free_list),
                    usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
                });

            let initial_state = GpuState {
                active_nodes: net.nodes.len() as u32,
                interactions: 0,
                free_list_head: 0,
                _padding: 0,
            };

            let state_buf = self
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("State Buffer"),
                    contents: bytemuck::cast_slice(&[initial_state]),
                    usage: wgpu::BufferUsages::STORAGE
                        | wgpu::BufferUsages::COPY_SRC
                        | wgpu::BufferUsages::COPY_DST,
                });

            let bind_group_layout = self.pipeline.get_bind_group_layout(0);
            let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Bind Group"),
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: arena_buf.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: free_list_buf.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: state_buf.as_entire_binding(),
                    },
                ],
            });

            let mut out_net = net.clone();
            let mut final_state = initial_state;
            let mut interactions_occurred = true;
            let mut max_iterations = 1000;

            while interactions_occurred && max_iterations > 0 {
                // Reset interaction counter for this pass
                let pass_state = GpuState {
                    active_nodes: final_state.active_nodes,
                    interactions: 0,
                    free_list_head: final_state.free_list_head,
                    _padding: 0,
                };

                self.queue
                    .write_buffer(&state_buf, 0, bytemuck::cast_slice(&[pass_state]));

                let mut encoder = self
                    .device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
                {
                    let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                        label: None,
                        timestamp_writes: None,
                    });
                    cpass.set_pipeline(&self.pipeline);
                    cpass.set_bind_group(0, &bind_group, &[]);
                    let workgroups = (out_net.nodes.len() as u32 + 63) / 64;
                    cpass.dispatch_workgroups(workgroups, 1, 1);
                }

                // Create staging buffer to read back just the state
                let staging_state = self.device.create_buffer(&wgpu::BufferDescriptor {
                    label: None,
                    size: std::mem::size_of::<GpuState>() as u64,
                    usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
                });

                encoder.copy_buffer_to_buffer(
                    &state_buf,
                    0,
                    &staging_state,
                    0,
                    staging_state.size(),
                );
                self.queue.submit(Some(encoder.finish()));

                let state_slice = staging_state.slice(..);
                let (tx, rx) = std::sync::mpsc::channel();
                state_slice.map_async(wgpu::MapMode::Read, move |v| tx.send(v).unwrap());
                self.device.poll(wgpu::Maintain::Wait);
                rx.recv().unwrap().unwrap();

                let state_data = state_slice.get_mapped_range();
                let current_state: GpuState = bytemuck::cast_slice(&state_data)[0];
                drop(state_data);
                staging_state.unmap();

                if current_state.interactions == 0 {
                    interactions_occurred = false;
                } else {
                    final_state.interactions += current_state.interactions;
                    final_state.active_nodes = current_state.active_nodes;
                    final_state.free_list_head = current_state.free_list_head;
                    max_iterations -= 1;
                }
            }

            let gc_bind_group_layout = self.gc_pipeline.get_bind_group_layout(0);
            let gc_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("GC Bind Group"),
                layout: &gc_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: arena_buf.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: free_list_buf.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: state_buf.as_entire_binding(),
                    },
                ],
            });

            // Dispatch isolated topological compute pass for Cycle Garbage Collection
            let mut gc_encoder =
                self.device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("GC Encoder"),
                    });
            {
                let mut gc_pass = gc_encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                    label: Some("GC Pass"),
                    timestamp_writes: None,
                });
                gc_pass.set_pipeline(&self.gc_pipeline);
                gc_pass.set_bind_group(0, &gc_bind_group, &[]);
                let workgroups = (out_net.nodes.len() as u32 + 63) / 64;
                gc_pass.dispatch_workgroups(workgroups, 1, 1);
            }
            self.queue.submit(Some(gc_encoder.finish()));

            // Create staging buffer for final readback
            let staging_arena = self.device.create_buffer(&wgpu::BufferDescriptor {
                label: None,
                size: (out_net.nodes.len() * std::mem::size_of::<crate::ast::Node>()) as u64,
                usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });

            let mut final_encoder = self
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
            final_encoder.copy_buffer_to_buffer(
                &arena_buf,
                0,
                &staging_arena,
                0,
                staging_arena.size(),
            );
            self.queue.submit(Some(final_encoder.finish()));

            let arena_slice = staging_arena.slice(..);
            let (tx, rx) = std::sync::mpsc::channel();
            arena_slice.map_async(wgpu::MapMode::Read, move |v| tx.send(v).unwrap());
            self.device.poll(wgpu::Maintain::Wait);
            rx.recv().unwrap().unwrap();

            let arena_data = arena_slice.get_mapped_range();
            let final_nodes: Vec<crate::ast::Node> = bytemuck::cast_slice(&arena_data).to_vec();
            drop(arena_data);
            staging_arena.unmap();

            out_net.nodes = final_nodes;

            (out_net, final_state)
        })
    }
}
