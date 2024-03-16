use crate::rules_engine::traits::argument::Argument;
use anyhow::Error as AnyError;

#[typetag::serde(tag = "type")]
pub trait FloatArgument: Argument {
  fn value(&self) -> Result<f64, AnyError>;
}

#[typetag::serde(name = "Float")]
impl Argument for f64 {}

#[typetag::serde(name = "Float")]
impl FloatArgument for f64 {
  fn value(&self) -> Result<f64, anyhow::Error> {
    Ok(*self)
  }
}
