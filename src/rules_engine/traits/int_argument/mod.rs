use crate::rules_engine::traits::argument::Argument;
use anyhow::Error as AnyError;

#[typetag::serde(tag = "type")]
pub trait IntArgument: Argument {
  fn value(&self) -> Result<i64, AnyError>;
}
