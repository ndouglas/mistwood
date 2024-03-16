use anyhow::Error as AnyError;

#[typetag::serde(tag = "type")]
pub trait Condition {
  fn is_met(&self) -> Result<bool, AnyError>;
}
