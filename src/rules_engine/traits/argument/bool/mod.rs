use anyhow::Error as AnyError;
use std::collections::HashMap;

#[typetag::serde(tag = "type")]
pub trait BoolArgument {
  fn evaluate(&self) -> Result<bool, AnyError>;
}

#[typetag::serde(name = "Bool")]
impl BoolArgument for bool {
  fn evaluate(&self) -> Result<bool, anyhow::Error> {
    Ok(*self)
  }
}

impl PartialEq for dyn BoolArgument {
  fn eq(&self, other: &Self) -> bool {
    self.evaluate().unwrap() == other.evaluate().unwrap()
  }
}

define_list_argument_trait_and_impl!(BoolListArgument, BoolArgument, "BoolList");
define_map_argument_trait_and_impl!(BoolMapArgument, BoolArgument, "BoolMap");
