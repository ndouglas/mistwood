use crate::prelude::FloatArgument;
use anyhow::Error as AnyError;
use std::collections::HashMap;

#[typetag::serde(name = "Float")]
impl FloatArgument for f64 {
  fn evaluate(&self) -> Result<f64, anyhow::Error> {
    Ok(*self)
  }
}

define_list_argument_trait_and_impl!(FloatListArgument, FloatArgument, "FloatList", f64);
define_map_argument_trait_and_impl!(FloatMapArgument, FloatArgument, "FloatMap", f64);
