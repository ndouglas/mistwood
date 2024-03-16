use anyhow::Error as AnyError;
use std::collections::HashMap;

#[typetag::serde(tag = "type")]
pub trait StringArgument {
  fn value(&self) -> Result<&String, AnyError>;
}

#[typetag::serde(name = "String")]
impl StringArgument for String {
  fn value(&self) -> Result<&String, anyhow::Error> {
    Ok(self)
  }
}

impl PartialEq for dyn StringArgument {
  fn eq(&self, other: &Self) -> bool {
    self.value().unwrap() == other.value().unwrap()
  }
}

define_list_argument_trait_and_impl!(StringListArgument, StringArgument, "StringList");
define_map_argument_trait_and_impl!(StringMapArgument, StringArgument, "StringMap");
