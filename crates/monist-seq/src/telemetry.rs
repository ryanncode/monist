use std::process::Output;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct BendTelemetry {
    pub interactions: u64,
    pub reductions: u64,
    pub execution_time: Duration,
    pub successful: bool,
    pub stdout: String,
    pub stderr: String,
}

pub struct TelemetryParser;

impl TelemetryParser {
    pub fn parse_output(output: Output) -> BendTelemetry {
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        
        let mut interactions = 0;
        let mut reductions = 0;
        let mut execution_time = Duration::from_secs(0);
        
        // Example output parsing logic for Bend's `-s` metrics.
        // Bend typically prints out metrics to stderr like:
        // "Interactions: 1204"
        // "Time: 0.05s"
        
        for line in stderr.lines() {
            if line.contains("Interactions:") {
                if let Some(num_str) = line.split(':').nth(1) {
                    if let Ok(num) = num_str.trim().parse::<u64>() {
                        interactions = num;
                    }
                }
            } else if line.contains("Reductions:") {
                if let Some(num_str) = line.split(':').nth(1) {
                    if let Ok(num) = num_str.trim().parse::<u64>() {
                        reductions = num;
                    }
                }
            } else if line.contains("Time:") {
                if let Some(time_str) = line.split(':').nth(1) {
                    let time_str = time_str.trim().trim_end_matches('s');
                    if let Ok(secs) = time_str.parse::<f64>() {
                        execution_time = Duration::from_secs_f64(secs);
                    }
                }
            }
        }
        
        BendTelemetry {
            interactions,
            reductions,
            execution_time,
            successful: output.status.success(),
            stdout,
            stderr,
        }
    }
}
