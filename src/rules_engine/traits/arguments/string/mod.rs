use crate::rules_engine::traits::argument::Argument;
use anyhow::Error as AnyError;

#[typetag::serde(tag = "type")]
pub trait StringArgument: Argument {
  fn value(&self) -> Result<&String, AnyError>;
}

#[typetag::serde(name = "String")]
impl Argument for String {}

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
