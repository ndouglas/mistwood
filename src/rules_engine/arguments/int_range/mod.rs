use crate::prelude::IntRangeArgument;
use anyhow::Error as AnyError;
use core::ops::Range;
use std::collections::HashMap;

#[typetag::serde(name = "IntRange")]
impl IntRangeArgument for Range<i64> {
  fn evaluate(&self) -> Result<Range<i64>, AnyError> {
    Ok(self.clone())
  }
}

define_list_argument_trait_and_impl!(IntRangeListArgument, IntRangeArgument, "IntRangeList", Range<i64>);
define_map_argument_trait_and_impl!(IntRangeMapArgument, IntRangeArgument, "IntRangeMap", Range<i64>);
define_script_argument_trait_and_string_impl!(IntScriptArgument, IntArgument, "IntScript", i64, mlua::Value::Integer(i) => Ok(i));
