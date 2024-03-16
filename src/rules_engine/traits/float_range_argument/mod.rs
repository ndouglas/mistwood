use crate::rules_engine::traits::argument::Argument;
use crate::rules_engine::traits::int_argument::IntArgument;
use anyhow::Error as AnyError;
use core::ops::Range;

#[typetag::serde(tag = "type")]
pub trait FloatRangeArgument: Argument {
  fn value(&self) -> Result<&Range<f64>, AnyError>;
}

#[typetag::serde]
impl Argument for Range<f64> {}

#[typetag::serde]
impl FloatRangeArgument for Range<f64> {
  fn value(&self) -> Result<&Range<f64>, AnyError> {
    Ok(self)
  }
}
