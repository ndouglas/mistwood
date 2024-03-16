use crate::rules_engine::traits::condition::Condition;
use crate::rules_engine::traits::uint_argument::UintArgument;
use anyhow::Error as AnyError;
use serde::{Deserialize, Serialize};

#[derive(Derivative, Serialize, Deserialize)]
#[derivative(Debug)]
pub struct UintNotEquals {
  #[derivative(Debug = "ignore")]
  pub left: Box<dyn UintArgument>,
  #[derivative(Debug = "ignore")]
  pub right: Box<dyn UintArgument>,
}

#[typetag::serde]
impl Condition for UintNotEquals {
  fn is_met(&self) -> Result<bool, AnyError> {
    Ok(self.left.value()? != self.right.value()?)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test::init as test_init;

  #[test]
  fn test_is_met() {
    test_init();
    let condition = UintNotEquals {
      left: Box::new(1),
      right: Box::new(1),
    };
    assert_eq!(condition.is_met().unwrap(), false);
  }

  #[test]
  fn test_is_met2() {
    test_init();
    let condition = UintNotEquals {
      left: Box::new(1),
      right: Box::new(2),
    };
    assert_eq!(condition.is_met().unwrap(), true);
  }

  #[test]
  fn test_serde() {
    test_init();
    let condition = &UintNotEquals {
      left: Box::new(1),
      right: Box::new(1),
    } as &dyn Condition;
    let serialized = serde_json::to_string(condition).unwrap();
    assert_eq!(
      serialized,
      r#"{"type":"UintNotEquals","left":{"type":"Uint","value":1},"right":{"type":"Uint","value":1}}"#
    );
    let deserialized: UintNotEquals = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized.is_met().unwrap(), false);
  }
}
