use anyhow::Error as AnyError;

#[typetag::serde(tag = "type")]
pub trait FloatValue {
  fn evaluate(&self) -> Result<f64, AnyError>;
}
