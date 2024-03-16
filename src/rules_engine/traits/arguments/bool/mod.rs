use crate::rules_engine::traits::argument::Argument;
use anyhow::Error as AnyError;
use std::collections::HashMap;

#[typetag::serde(tag = "type")]
pub trait BoolArgument: Argument {
  fn value(&self) -> Result<bool, AnyError>;
}

#[typetag::serde]
impl Argument for bool {}

#[typetag::serde(name = "Bool")]
impl BoolArgument for bool {
  fn value(&self) -> Result<bool, anyhow::Error> {
    Ok(*self)
  }
}

impl PartialEq for dyn BoolArgument {
  fn eq(&self, other: &Self) -> bool {
    self.value().unwrap() == other.value().unwrap()
  }
}

define_list_argument_trait_and_impl!(BoolListArgument, BoolArgument, "BoolList");
define_map_argument_trait_and_impl!(BoolMapArgument, BoolArgument, "BoolMap");
