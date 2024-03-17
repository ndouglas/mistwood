use crate::prelude::IntValue;
use anyhow::Error as AnyError;

#[typetag::serde(tag = "type")]
pub trait IntOperation {
  fn execute(&self) -> Result<Box<dyn IntValue>, AnyError>;
}
