use crate::rules_engine::traits::argument::Argument;
use anyhow::Error as AnyError;

#[typetag::serde(tag = "type")]
pub trait UintArgument: Argument {
  fn value(&self) -> Result<u64, AnyError>;
}

#[typetag::serde(name = "Uint")]
impl Argument for u64 {}

#[typetag::serde(name = "Uint")]
impl UintArgument for u64 {
  fn value(&self) -> Result<u64, anyhow::Error> {
    Ok(*self)
  }
}
