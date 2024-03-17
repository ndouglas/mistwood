use crate::prelude::Condition;
use crate::prelude::Context;
use anyhow::Error as AnyError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Error {}

#[typetag::serde]
impl Condition for Error {
  fn is_met(&self, _context: &Box<dyn Context>) -> Result<bool, AnyError> {
    Err(anyhow!("This condition always returns an error."))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::prelude::NullContext;
  use crate::test::init as test_init;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_is_met() {
    test_init();
    let condition = Error {};
    let context = Box::new(NullContext) as Box<dyn Context>;
    assert!(condition.is_met(&context).is_err());
  }

  #[test]
  fn test_serde() {
    test_init();
    let condition = &Error {} as &dyn Condition;
    let serialized = serde_yaml::to_string(condition).unwrap();
    println!("{}", serialized);
    assert_eq!(
      serialized.trim(),
      r#"
type: Error
          "#
      .trim()
    );
    let deserialized: Box<dyn Condition> = serde_yaml::from_str(&serialized).unwrap();
    let context = Box::new(NullContext) as Box<dyn Context>;
    assert!(deserialized.is_met(&context).is_err());
  }
}
