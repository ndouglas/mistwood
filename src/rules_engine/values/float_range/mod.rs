use crate::prelude::FloatRangeArgument;
use anyhow::Error as AnyError;
use core::ops::Range;
use std::collections::HashMap;

#[typetag::serde(name = "FloatRange")]
impl FloatRangeArgument for Range<f64> {
  fn evaluate(&self) -> Result<Range<f64>, AnyError> {
    Ok(self.clone())
  }
}

define_list_argument_trait_and_impl!(FloatRangeListArgument, FloatRangeArgument, "FloatRangeList", Range<f64>);
define_map_argument_trait_and_impl!(FloatRangeMapArgument, FloatRangeArgument, "FloatRangeMap", Range<f64>);
define_script_argument_trait_and_string_impl!(FloatScriptValue, FloatValue, "FloatScript", f64, mlua::Value::Number(n) => Ok(n));
