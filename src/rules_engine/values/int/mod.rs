use crate::prelude::IntValue;
use anyhow::Error as AnyError;
use std::collections::HashMap;

#[typetag::serde(name = "Int")]
impl IntValue for i64 {
  fn evaluate(&self) -> Result<i64, anyhow::Error> {
    Ok(*self)
  }
}

define_list_argument_trait_and_impl!(IntListValue, IntValue, "IntList", i64);
define_map_argument_trait_and_impl!(IntMapValue, IntValue, "IntMap", i64);
