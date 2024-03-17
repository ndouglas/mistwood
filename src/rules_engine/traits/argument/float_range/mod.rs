use anyhow::Error as AnyError;
use core::ops::Range;

#[typetag::serde(tag = "type")]
pub trait FloatRangeArgument {
  fn evaluate(&self) -> Result<Range<f64>, AnyError>;
}
