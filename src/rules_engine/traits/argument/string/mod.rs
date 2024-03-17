use anyhow::Error as AnyError;
use std::collections::HashMap;

#[typetag::serde(tag = "type")]
pub trait StringArgument {
  fn evaluate(&self) -> Result<String, AnyError>;
}

#[typetag::serde(name = "String")]
impl StringArgument for String {
  fn evaluate(&self) -> Result<String, anyhow::Error> {
    Ok(self.clone())
  }
}

impl PartialEq for dyn StringArgument {
  fn eq(&self, other: &Self) -> bool {
    self.evaluate().unwrap() == other.evaluate().unwrap()
  }
}

define_list_argument_trait_and_impl!(StringListArgument, StringArgument, "StringList");
define_map_argument_trait_and_impl!(StringMapArgument, StringArgument, "StringMap");
