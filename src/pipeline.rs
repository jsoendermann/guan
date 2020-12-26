use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::process::Command;

#[derive(Serialize, Deserialize, Debug)]
pub struct Stage {
  pub id: String,
  pub name: String,
  pub run: String,
  pub depends_on: Option<Vec<String>>,
}

impl Stage {
  pub fn run(&self, workdir: &str) -> Result<String, String> {
    let output = Command::new("bash")
      .arg("-c")
      .arg(&self.run)
      .current_dir(workdir)
      .output()
      .expect("Error running command");

    if output.status.success() {
      let stdout = String::from_utf8(output.stdout).expect("Error reading stdout");
      Ok(stdout)
    } else {
      let stderr = String::from_utf8(output.stderr).expect("Error reading stderr");
      Err(stderr)
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pipeline {
  pub name: String,
  pub stages: Vec<Stage>,
}

impl Pipeline {
  pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Pipeline, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let p = serde_yaml::from_reader(reader)?;
    Ok(p)
  }
}
