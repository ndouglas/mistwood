use crate::rules_engine::traits::argument::Argument;
use crate::rules_engine::traits::bool_argument::BoolArgument;
use crate::rules_engine::traits::bool_operation::BoolOperation;
use crate::rules_engine::traits::operation::Operation;
use anyhow::Error as AnyError;

#[derive(Serialize, Deserialize, Derivative)]
#[derivative(Debug)]
pub struct Not {
  #[derivative(Debug = "ignore")]
  value: Box<dyn BoolArgument>,
}

#[typetag::serde]
impl Operation for Not {}

#[typetag::serde]
impl BoolOperation for Not {
  fn execute(&self) -> Result<Box<dyn BoolArgument>, AnyError> {
    Ok(Box::new(!self.value.value()?))
  }
}

#[typetag::serde]
impl Argument for Not {}

#[typetag::serde]
impl BoolArgument for Not {
  fn value(&self) -> Result<bool, AnyError> {
    self.execute()?.value()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_execute() {
    let operation = Not { value: Box::new(true) };
    assert!(!operation.execute().unwrap().value().unwrap());
  }

  #[test]
  fn test_serde() {
    let operation = &Not { value: Box::new(true) } as &dyn BoolOperation;
    let serialized = serde_json::to_string(operation).unwrap();
    assert_eq!(serialized, r#"{"type":"Not","value":{"type":"Bool","value":true}}"#);
    let deserialized: Box<dyn BoolOperation> = serde_json::from_str(&serialized).unwrap();
    assert_eq!(
      operation.execute().unwrap().value().unwrap(),
      deserialized.execute().unwrap().value().unwrap()
    );
  }
}
