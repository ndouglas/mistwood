use anyhow::Error as AnyError;

#[typetag::serde(tag = "type")]
pub trait IntArgument {
  fn evaluate(&self) -> Result<i64, AnyError>;
}
