use anyhow::Error as AnyError;

#[typetag::serde(tag = "type")]
pub trait IntValue {
  fn evaluate(&self) -> Result<i64, AnyError>;
}
