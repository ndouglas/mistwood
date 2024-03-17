use anyhow::Error as AnyError;
use core::ops::Range;
use std::collections::HashMap;

#[typetag::serde(tag = "type")]
pub trait FloatArgument {
  fn value(&self) -> Result<f64, AnyError>;
}

#[typetag::serde(name = "Float")]
impl FloatArgument for f64 {
  fn value(&self) -> Result<f64, anyhow::Error> {
    Ok(*self)
  }
}

define_list_argument_trait_and_impl!(FloatListArgument, FloatArgument, "FloatList");
define_map_argument_trait_and_impl!(FloatMapArgument, FloatArgument, "FloatMap");
define_script_argument_impl!(FloatScriptArgument, FloatArgument, "FloatScript", f64, Number);

#[typetag::serde(tag = "type")]
pub trait FloatRangeArgument {
  fn value(&self) -> Result<&Range<f64>, AnyError>;
}

#[typetag::serde(name = "FloatRange")]
impl FloatRangeArgument for Range<f64> {
  fn value(&self) -> Result<&Range<f64>, AnyError> {
    Ok(self)
  }
}

define_list_argument_trait_and_impl!(FloatRangeListArgument, FloatRangeArgument, "FloatRangeList");
define_map_argument_trait_and_impl!(FloatRangeMapArgument, FloatRangeArgument, "FloatRangeMap");
