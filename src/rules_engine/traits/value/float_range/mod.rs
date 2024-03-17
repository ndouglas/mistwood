use anyhow::Error as AnyError;
use core::ops::Range;

#[typetag::serde(tag = "type")]
pub trait FloatRangeValue {
  fn evaluate(&self) -> Result<Range<f64>, AnyError>;
}

value_list_trait!(FloatRangeListValue, Range<f64>);
value_map_trait!(FloatRangeMapValue, Range<f64>);
