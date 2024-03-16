use crate::rules_engine::traits::condition::Condition;
use crate::rules_engine::traits::int_argument::IntArgument;
use anyhow::Error as AnyError;
use serde::{Deserialize, Serialize};

#[derive(Derivative, Serialize, Deserialize)]
#[derivative(Debug)]
pub struct IntGreaterThanOrEquals {
  #[derivative(Debug = "ignore")]
  pub left: Box<dyn IntArgument>,
  #[derivative(Debug = "ignore")]
  pub right: Box<dyn IntArgument>,
}

#[typetag::serde]
impl Condition for IntGreaterThanOrEquals {
  fn is_met(&self) -> Result<bool, AnyError> {
    Ok(self.left.value()? >= self.right.value()?)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test::init as test_init;

  #[test]
  fn test_is_met() {
    test_init();
    let condition = IntGreaterThanOrEquals {
      left: Box::new(1),
      right: Box::new(1),
    };
    assert!(condition.is_met().unwrap());
  }

  #[test]
  fn test_is_met2() {
    test_init();
    let condition = IntGreaterThanOrEquals {
      left: Box::new(1),
      right: Box::new(2),
    };
    assert!(!condition.is_met().unwrap());
  }

  #[test]
  fn test_is_met3() {
    test_init();
    let condition = IntGreaterThanOrEquals {
      left: Box::new(2),
      right: Box::new(1),
    };
    assert!(condition.is_met().unwrap());
  }

  #[test]
  fn test_serde() {
    test_init();
    let condition = &IntGreaterThanOrEquals {
      left: Box::new(2),
      right: Box::new(1),
    } as &dyn Condition;
    let serialized = serde_json::to_string(condition).unwrap();
    assert_eq!(
      serialized,
      r#"{"type":"IntGreaterThanOrEquals","left":{"type":"Int","value":2},"right":{"type":"Int","value":1}}"#
    );
    let deserialized: IntGreaterThanOrEquals = serde_json::from_str(&serialized).unwrap();
    assert!(deserialized.is_met().unwrap());
  }
}
