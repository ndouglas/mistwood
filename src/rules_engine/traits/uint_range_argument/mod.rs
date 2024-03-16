use crate::rules_engine::traits::argument::Argument;
use anyhow::Error as AnyError;
use core::ops::Range;

#[typetag::serde(tag = "type")]
pub trait UintRangeArgument: Argument {
  fn value(&self) -> Result<&Range<u64>, AnyError>;
}

#[typetag::serde(name = "UintRange")]
impl Argument for Range<u64> {}

#[typetag::serde(name = "UintRange")]
impl UintRangeArgument for Range<u64> {
  fn value(&self) -> Result<&Range<u64>, AnyError> {
    Ok(self)
  }
}
