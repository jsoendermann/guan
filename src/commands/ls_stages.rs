use std::error::Error;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use super::command::GuanCommand;
use crate::pipeline::Pipeline;

pub struct LsStagesArgs {
  pub pipeline_file_path: String,
}

pub struct LsStagesCommand {
  args: LsStagesArgs,
}

impl LsStagesCommand {
  pub fn new(args: LsStagesArgs) -> LsStagesCommand {
    LsStagesCommand { args }
  }
}

impl GuanCommand for LsStagesCommand {
  fn execute(&self) -> Result<(), Box<dyn Error>> {
    let pipeline = Pipeline::load_from_file(&self.args.pipeline_file_path)
      .expect("Can't open pipeline definition file");

    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    for stage in pipeline.stages.iter() {
      stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
      write!(&mut stdout, "{}", stage.id)?;
      stdout.reset()?;
      println!(" {}", stage.name);
    }

    Ok(())
  }
}
