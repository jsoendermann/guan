use super::command::GuanCommand;
use crate::pipeline::Pipeline;

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
      println!("Running {}", stage.name);
      match stage.run(&self.args.workdir) {
        Err(stderr) => {
          println!("Something went wrong. Stderr:\n\n{}", stderr);
          break;
        }
        _ => {
          println!("Done!");
        }
      }
    }

    Ok(())
  }
}
