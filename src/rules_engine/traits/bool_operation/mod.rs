use crate::rules_engine::traits::bool_argument::BoolArgument;
use anyhow::Error as AnyError;

#[typetag::serde(tag = "type")]
pub trait BoolOperation {
  fn execute(&self) -> Result<Box<dyn BoolArgument>, AnyError>;
}
