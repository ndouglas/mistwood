use crate::rules_engine::traits::argument::Argument;
use anyhow::Error as AnyError;
use specs::prelude::*;

#[typetag::serde(tag = "type")]
pub trait EntityArgument: Argument {
  fn value(&self) -> Result<&Entity, AnyError>;
}
