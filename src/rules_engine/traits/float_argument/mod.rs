use crate::rules_engine::traits::argument::Argument;
use anyhow::Error as AnyError;

#[typetag::serde(tag = "type")]
pub trait FloatArgument: Argument {
  fn value(&self) -> Result<f64, AnyError>;
}
