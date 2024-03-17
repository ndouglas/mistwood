use anyhow::Error as AnyError;
use core::ops::Range;

#[typetag::serde(tag = "type")]
pub trait IntRangeArgument {
  fn evaluate(&self) -> Result<Range<i64>, AnyError>;
}
