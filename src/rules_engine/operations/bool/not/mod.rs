use crate::prelude::BoolArgument;
use crate::prelude::BoolOperation;
use anyhow::Error as AnyError;

#[derive(Serialize, Deserialize, Derivative)]
#[derivative(Debug)]
pub struct Not {
  #[derivative(Debug = "ignore")]
  value: Box<dyn BoolArgument>,
}

#[typetag::serde]
impl BoolOperation for Not {
  fn execute(&self) -> Result<Box<dyn BoolArgument>, AnyError> {
    Ok(Box::new(!self.value.value()?))
  }
}

define_argument_for_operation!(BoolArgument, Not, bool);

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_execute() {
    let operation = Not { value: Box::new(true) };
    assert!(!operation.execute().unwrap().value().unwrap());
  }

  #[test]
  fn test_serde() {
    let operation = &Not { value: Box::new(true) } as &dyn BoolOperation;
    let serialized = serde_yaml::to_string(operation).unwrap();
    println!("{}", serialized);
    assert_eq!(
      serialized.trim(),
      r#"
type: Not
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
