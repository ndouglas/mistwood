use crate::rules_engine::traits::argument::Argument;
use anyhow::Error as AnyError;

#[typetag::serde(tag = "type")]
pub trait ListArgument: Argument {
  fn value(&self) -> Result<Vec<Box<dyn Argument>>, AnyError>;
}
