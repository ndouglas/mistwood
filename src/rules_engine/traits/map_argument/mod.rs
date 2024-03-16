use crate::rules_engine::traits::argument::Argument;
use anyhow::Error as AnyError;
use std::collections::HashMap;

#[typetag::serde(tag = "type")]
pub trait MapArgument: Argument {
  fn value(&self) -> Result<HashMap<String, Box<dyn Argument>>, AnyError>;
}
