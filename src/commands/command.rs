use std::error::Error;

pub trait GuanCommand {
  fn execute(&self) -> Result<(), Box<dyn Error>>;
}
