use anyhow::Error as AnyError;
use core::ops::Range;

#[typetag::serde(tag = "type")]
pub trait IntRangeValue {
  fn evaluate(&self) -> Result<Range<i64>, AnyError>;
}

value_list_trait!(IntRangeListValue, Range<i64>);
value_map_trait!(IntRangeMapValue, Range<i64>);
