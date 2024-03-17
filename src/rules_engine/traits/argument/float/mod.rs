use anyhow::Error as AnyError;

#[typetag::serde(tag = "type")]
pub trait FloatArgument {
  fn evaluate(&self) -> Result<f64, AnyError>;
}
