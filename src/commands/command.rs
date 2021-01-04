use std::error::Error;

pub trait GuanCommand {
  type Args;

  fn new(args: Self::Args) -> Self
  where
    Self: Sized;
  fn execute(&self) -> Result<(), Box<dyn Error>>;
}
