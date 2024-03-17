use crate::prelude::FloatRangeListValue;
use crate::prelude::FloatRangeMapValue;
use crate::prelude::FloatRangeValue;
use anyhow::Error as AnyError;
use core::ops::Range;

#[typetag::serde(name = "FloatRange")]
impl FloatRangeValue for Range<f64> {
  fn evaluate(&self) -> Result<Range<f64>, AnyError> {
    Ok(self.clone())
  }
}

value_list_impl!(FloatRangeListValue, FloatRangeValue, "FloatRangeList", Range<f64>);
value_map_impl!(FloatRangeMapValue, FloatRangeValue, "FloatRangeMap", Range<f64>);
