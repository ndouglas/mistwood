use anyhow::Error as AnyError;

#[typetag::serde(tag = "type")]
pub trait EnumValue {
  fn evaluate(&self) -> Result<String, AnyError>;
}
