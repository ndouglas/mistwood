use crate::rules_engine::traits::int_argument::IntArgument;
use anyhow::Error as AnyError;

#[typetag::serde(tag = "type")]
pub trait IntOperation {
  fn execute(&self) -> Result<Box<dyn IntArgument>, AnyError>;
}
