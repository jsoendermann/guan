use std::error::Error;

use super::command::GuanCommand;
use crate::pipeline::Pipeline;

pub struct RunStageArgs {
  pub stage_name: String,
  pub pipeline_file_path: String,
  pub workdir: String,
}

pub struct RunStageCommand {
  args: RunStageArgs,
}

impl GuanCommand for RunStageCommand {
  type Args = RunStageArgs;

  fn new(args: RunStageArgs) -> RunStageCommand {
    RunStageCommand { args }
  }

  fn execute(&self) -> Result<(), Box<dyn Error>> {
    let pipeline = Pipeline::from_file(&self.args.pipeline_file_path)?;

    let stage = pipeline
      .stages
      .iter()
      .find(|&s| s.id == self.args.stage_name)
      .expect("Can't find stage.");

    println!("Running {}...", stage.name);
    match stage.run(&self.args.workdir[..]) {
      Ok(_) => {
        println!("Success");
        Ok(())
      }
      Err(error) => {
        println!("Something went wrong\n\n{}", error);
        Err(error.into())
      }
    }
  }
}
