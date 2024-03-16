use crate::rules_engine::traits::argument::Argument;
use anyhow::Error as AnyError;

#[typetag::serde(tag = "type")]
pub trait IntArgument: Argument {
  fn value(&self) -> Result<i64, AnyError>;
}

#[typetag::serde(name = "Int")]
impl Argument for i64 {}

#[typetag::serde(name = "Int")]
impl IntArgument for i64 {
  fn value(&self) -> Result<i64, anyhow::Error> {
    Ok(*self)
  }
}
