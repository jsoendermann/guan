use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::process::Command;

#[derive(Serialize, Deserialize, Debug)]
struct Stage {
    name: String,
    run: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Pipeline {
    name: String,
    stages: Vec<Stage>,
}

fn main() {
    let pipeline = read_pipeline_from_file("pipeline.yml").expect("Can't read pipeline");
    println!("Deploying {}", pipeline.name);

    for stage in pipeline.stages.iter() {
        match execute_stage(stage) {
            Err(stderr) => {
                println!("Something went wrong. Stderr:\n\n{}", stderr);
                break;
            }
            _ => (),
        }
    }
}

fn read_pipeline_from_file<P: AsRef<Path>>(path: P) -> Result<Pipeline, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let p = serde_yaml::from_reader(reader)?;
    Ok(p)
}

fn execute_stage(stage: &Stage) -> Result<(), String> {
    println!("Running {}", stage.name);
    let output = Command::new("bash")
        .arg("-c")
        .arg(&stage.run)
        .output()
        .expect("Error running command");

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8(output.stderr).expect("Error reading stderr");
        Err(stderr)
    }
}
