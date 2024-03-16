use crate::rules_engine::traits::argument::Argument;
use crate::rules_engine::traits::int_argument::IntArgument;
use anyhow::Error as AnyError;

#[typetag::serde(tag = "type")]
pub trait IntListArgument: Argument {
  fn value(&self) -> Result<&Vec<Box<dyn IntArgument>>, AnyError>;
}

#[typetag::serde(name = "IntList")]
impl Argument for Vec<Box<dyn IntArgument>> {}

#[typetag::serde(name = "IntList")]
impl IntListArgument for Vec<Box<dyn IntArgument>> {
  fn value(&self) -> Result<&Vec<Box<dyn IntArgument>>, AnyError> {
    Ok(self)
  }
}
