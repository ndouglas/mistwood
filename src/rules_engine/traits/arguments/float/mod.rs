use crate::rules_engine::traits::argument::Argument;
use anyhow::Error as AnyError;
use core::ops::Range;

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

define_list_argument_trait_and_impl!(FloatListArgument, FloatArgument, "FloatList");

#[typetag::serde(tag = "type")]
pub trait FloatRangeArgument: Argument {
  fn value(&self) -> Result<&Range<f64>, AnyError>;
}

#[typetag::serde(name = "FloatRange")]
impl Argument for Range<f64> {}

#[typetag::serde(name = "FloatRange")]
impl FloatRangeArgument for Range<f64> {
  fn value(&self) -> Result<&Range<f64>, AnyError> {
    Ok(self)
  }
}

define_list_argument_trait_and_impl!(FloatRangeListArgument, FloatRangeArgument, "FloatRangeList");
