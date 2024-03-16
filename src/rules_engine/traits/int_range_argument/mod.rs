use crate::rules_engine::traits::argument::Argument;
use anyhow::Error as AnyError;
use core::ops::Range;

#[typetag::serde(tag = "type")]
pub trait IntRangeArgument: Argument {
  fn value(&self) -> Result<&Range<i64>, AnyError>;
}

#[typetag::serde]
impl Argument for Range<i64> {}

#[typetag::serde]
impl IntRangeArgument for Range<i64> {
  fn value(&self) -> Result<&Range<i64>, AnyError> {
    Ok(self)
  }
}
