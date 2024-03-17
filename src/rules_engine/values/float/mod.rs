use crate::prelude::FloatValue;
use anyhow::Error as AnyError;
use std::collections::HashMap;

#[typetag::serde(name = "Float")]
impl FloatValue for f64 {
  fn evaluate(&self) -> Result<f64, anyhow::Error> {
    Ok(*self)
  }
}

define_list_argument_trait_and_impl!(FloatListValue, FloatValue, "FloatList", f64);
define_map_argument_trait_and_impl!(FloatMapValue, FloatValue, "FloatMap", f64);
