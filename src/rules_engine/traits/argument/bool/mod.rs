use anyhow::Error as AnyError;

#[typetag::serde(tag = "type")]
pub trait BoolValue {
  fn evaluate(&self) -> Result<bool, AnyError>;
}
