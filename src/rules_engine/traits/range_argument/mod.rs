use crate::rules_engine::traits::argument::Argument;
use crate::rules_engine::traits::int_argument::IntArgument;
use anyhow::Error as AnyError;

#[typetag::serde(tag = "type")]
pub trait RangeArgument: Argument {
  fn value(&self) -> Result<(Box<dyn IntArgument>, Box<dyn IntArgument>), AnyError>;
}
