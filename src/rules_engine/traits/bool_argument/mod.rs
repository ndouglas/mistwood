use crate::rules_engine::traits::argument::Argument;
use anyhow::Error as AnyError;

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
