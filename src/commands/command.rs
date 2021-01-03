pub trait GuanCommand {
  type Args;

  fn new(args: Self::Args) -> Self
  where
    Self: Sized;
  fn execute(&self) -> Result<(), String>;
}
