use std::time::Duration;
use monist_comb::backend::GpuState;

#[derive(Debug, Clone)]
pub struct WgpuTelemetry {
    pub interactions: u64,
    pub reductions: u64,
    pub execution_time: Duration,
    pub successful: bool,
}

pub struct TelemetryParser;

impl TelemetryParser {
    pub fn parse_output(state: &GpuState, time: Duration) -> WgpuTelemetry {
        WgpuTelemetry {
            interactions: state.interactions as u64,
            reductions: state.interactions as u64, // simplified for now
            execution_time: time,
            successful: true, // WGPU executor handles errors earlier
        }
    }
}
