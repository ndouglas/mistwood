use anyhow::Error as AnyError;

#[typetag::serde(tag = "type")]
pub trait BoolArgument {
  fn evaluate(&self) -> Result<bool, AnyError>;
}
