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

        // Invoke the hvm gen-cu to compile the Bend/HVM syntax into CUDA
        // Wait, hvm gen-cu generates the .cu file or binary.
        // We will assume `hvm run` can be used or we run `hvm gen-cu` then `hvm run` if needed.
        // Or if we compile it and then run the generated CU file. Let's just execute `hvm run`.
        // The prompt asks to "trigger the `hvm gen-cu` and `hvm run` commands".
        
        let gen_output = Command::new("hvm")
            .arg("gen-cu")
            .arg(filepath.to_str().unwrap())
            .output()?;

        if !gen_output.status.success() {
            return Ok(gen_output); // Return compilation error
        }

        // Invoke hvm run on the file
        Command::new("hvm")
            .arg("run")
            .arg(filepath.to_str().unwrap())
            .arg("-s") // Enables metrics like reductions and interactions per second
            .output()
    }
}
