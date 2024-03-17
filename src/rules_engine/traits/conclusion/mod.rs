use crate::prelude::Context;
use anyhow::Error as AnyError;

#[typetag::serde(tag = "type")]
pub trait Conclusion {
  fn execute(&self, context: &Box<dyn Context>) -> Result<(), AnyError>;
}
