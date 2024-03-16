use crate::rules_engine::traits::argument::Argument;
use anyhow::Error as AnyError;

#[typetag::serde(tag = "type")]
pub trait StringArgument: Argument {
  fn value(&self) -> Result<String, AnyError>;
}
