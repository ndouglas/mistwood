use crate::prelude::IntArgument;
use anyhow::Error as AnyError;
use std::collections::HashMap;

#[typetag::serde(name = "Int")]
impl IntArgument for i64 {
  fn evaluate(&self) -> Result<i64, anyhow::Error> {
    Ok(*self)
  }
}

define_list_argument_trait_and_impl!(IntListArgument, IntArgument, "IntList", i64);
define_map_argument_trait_and_impl!(IntMapArgument, IntArgument, "IntMap", i64);
