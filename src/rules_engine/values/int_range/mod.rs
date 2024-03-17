use crate::prelude::IntRangeListValue;
use crate::prelude::IntRangeMapValue;
use crate::prelude::IntRangeValue;
use anyhow::Error as AnyError;
use core::ops::Range;

#[typetag::serde(name = "IntRange")]
impl IntRangeValue for Range<i64> {
  fn evaluate(&self) -> Result<Range<i64>, AnyError> {
    Ok(self.clone())
  }
}

value_list_impl!(IntRangeListValue, IntRangeValue, "IntRangeList", Range<i64>);
value_map_impl!(IntRangeMapValue, IntRangeValue, "IntRangeMap", Range<i64>);
