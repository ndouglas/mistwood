use crate::rules_engine::traits::argument::Argument;
use anyhow::Error as AnyError;
use std::collections::HashMap;

#[typetag::serde(tag = "type")]
pub trait MapArgument: Argument {
  fn value(&self) -> Result<&HashMap<String, Box<dyn Argument>>, AnyError>;
}

#[typetag::serde(name = "Map")]
impl Argument for HashMap<String, Box<dyn Argument>> {}

#[typetag::serde(name = "Map")]
impl MapArgument for HashMap<String, Box<dyn Argument>> {
  fn value(&self) -> Result<&HashMap<String, Box<dyn Argument>>, AnyError> {
    Ok(self)
  }
}
