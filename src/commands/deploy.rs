use super::command::GuanCommand;
use crate::pipeline::{Pipeline, Stage};

use std::process::Command;

pub struct DeployArgs {
  pub pipeline_file_path: String,
  pub workdir: String,
}

pub struct DeployCommand {
  args: DeployArgs,
}

impl GuanCommand for DeployCommand {
  type Args = DeployArgs;

  fn new(args: DeployArgs) -> DeployCommand {
    DeployCommand { args }
  }

  fn execute(&self) -> Result<(), String> {
    let pipeline = Pipeline::from_file(&self.args.pipeline_file_path)
      .expect("Can't open pipeline definition file");

    for stage in pipeline.stages.iter() {
      match self.execute_stage(stage) {
        Err(stderr) => {
          println!("Something went wrong. Stderr:\n\n{}", stderr);
          break;
        }
        _ => (),
      }
    }

    Ok(())
  }
}

impl DeployCommand {
  fn execute_stage(&self, stage: &Stage) -> Result<(), String> {
    println!("Running {}", stage.name);
    let output = Command::new("bash")
      .arg("-c")
      .arg(&stage.run)
      .current_dir(&self.args.workdir)
      .output()
      .expect("Error running command");

    if output.status.success() {
      Ok(())
    } else {
      let stderr = String::from_utf8(output.stderr).expect("Error reading stderr");
      Err(stderr)
    }
  }
}
