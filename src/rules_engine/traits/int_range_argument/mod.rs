use crate::rules_engine::traits::argument::Argument;
use crate::rules_engine::traits::int_argument::IntArgument;
use anyhow::Error as AnyError;
use core::ops::Range;

#[typetag::serde(tag = "type")]
pub trait IntRangeArgument: Argument {
  fn value(&self) -> Result<(Box<dyn IntArgument>, Box<dyn IntArgument>), AnyError>;
}
