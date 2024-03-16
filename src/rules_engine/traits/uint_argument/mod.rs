use crate::rules_engine::traits::argument::Argument;
use anyhow::Error as AnyError;

#[typetag::serde(tag = "type")]
pub trait UintArgument: Argument {
  fn value(&self) -> Result<u64, AnyError>;
}
