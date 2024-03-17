use anyhow::Error as AnyError;

#[typetag::serde(tag = "type")]
pub trait EnumArgument {
  fn evaluate(&self) -> Result<String, AnyError>;
}
