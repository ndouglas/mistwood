use crate::prelude::Context;
use anyhow::Error as AnyError;

#[typetag::serde(tag = "type")]
pub trait Condition {
  fn is_met(&self, context: &dyn Context) -> Result<bool, AnyError>;
}
