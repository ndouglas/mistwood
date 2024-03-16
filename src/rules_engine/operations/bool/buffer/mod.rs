use crate::prelude::BoolArgument;
use crate::prelude::BoolOperation;
use crate::rules_engine::traits::operation::Operation;
use anyhow::Error as AnyError;

#[derive(Serialize, Deserialize, Derivative)]
#[derivative(Debug)]
pub struct Buffer {
  #[derivative(Debug = "ignore")]
  value: Box<dyn BoolArgument>,
}

#[typetag::serde]
impl Operation for Buffer {}

#[typetag::serde]
impl BoolOperation for Buffer {
  fn execute(&self) -> Result<Box<dyn BoolArgument>, AnyError> {
    Ok(Box::new(self.value.value()?))
  }
}

#[typetag::serde]
impl BoolArgument for Buffer {
  fn value(&self) -> Result<bool, AnyError> {
    self.execute()?.value()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_execute() {
    let operation = Buffer { value: Box::new(true) };
    assert!(operation.execute().unwrap().value().unwrap());
  }

  #[test]
  fn test_serde() {
    let operation = &Buffer { value: Box::new(true) } as &dyn BoolOperation;
    let serialized = serde_yaml::to_string(operation).unwrap();
    println!("{}", serialized);
    assert_eq!(
      serialized.trim(),
      r#"
type: Buffer
value:
  type: Bool
  value: true
    "#
      .trim()
    );
    let deserialized: Box<dyn BoolOperation> = serde_yaml::from_str(&serialized).unwrap();
    assert_eq!(
      operation.execute().unwrap().value().unwrap(),
      deserialized.execute().unwrap().value().unwrap()
    );
  }
}
