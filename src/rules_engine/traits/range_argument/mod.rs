use crate::rules_engine::traits::argument::Argument;
use anyhow::Error as AnyError;

#[typetag::serde(tag = "type")]
pub trait RangeArgument: Argument {
  fn value(&self) -> Result<(Box<dyn Argument>, Box<dyn Argument>), AnyError>;
}
