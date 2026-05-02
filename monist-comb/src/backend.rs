use std::process::{Command, Output};
use std::fs;
use std::io::Result;
use std::path::PathBuf;

pub struct BendExecutor {
    temp_dir: PathBuf,
}

impl BendExecutor {
    pub fn new(temp_dir: impl Into<PathBuf>) -> Self {
        BendExecutor {
            temp_dir: temp_dir.into(),
        }
    }

    pub fn compile_and_run_cuda(&self, filename: &str, logic_source: &str) -> Result<Output> {
        // Ensure the directory exists
        fs::create_dir_all(&self.temp_dir)?;
        
        let mut filepath = self.temp_dir.clone();
        filepath.push(format!("{}.bend", filename));
        
        // Write the validated combinatorial logic to a Bend file
        fs::write(&filepath, logic_source)?;

        // Invoke the Bend CUDA interpreter for massively parallel execution
        Command::new("bend")
            .arg("run-cu")
            .arg(filepath.to_str().unwrap())
            .arg("-s") // Enables metrics like reductions and interactions per second
            .output()
    }
}
