pub trait GuanCommand {
  type Args;

  fn new(args: Self::Args) -> Self;
  fn execute(&self) -> Result<(), String>;
}
