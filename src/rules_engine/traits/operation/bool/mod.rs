use crate::prelude::BoolValue;
use anyhow::Error as AnyError;

#[typetag::serde(tag = "type")]
pub trait BoolOperation {
  fn execute(&self) -> Result<Box<dyn BoolValue>, AnyError>;
}
