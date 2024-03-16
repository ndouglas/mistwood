use anyhow::Error as AnyError;

#[typetag::serde(tag = "type")]
pub trait Conclusion {
  fn execute(&self) -> Result<(), AnyError>;
}
