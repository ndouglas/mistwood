use crate::rules_engine::traits::bool_argument::BoolArgument;
use crate::rules_engine::traits::condition::Condition;
use anyhow::Error as AnyError;
use serde::{Deserialize, Serialize};

#[derive(Derivative, Serialize, Deserialize)]
#[derivative(Debug)]
pub struct BoolNotEquals {
  #[derivative(Debug = "ignore")]
  pub left: Box<dyn BoolArgument>,
  #[derivative(Debug = "ignore")]
  pub right: Box<dyn BoolArgument>,
}

#[typetag::serde]
impl Condition for BoolNotEquals {
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
    let condition = BoolNotEquals {
      left: Box::new(true),
      right: Box::new(true),
    };
    assert!(!condition.is_met().unwrap());
  }

  #[test]
  fn test_is_met2() {
    test_init();
    let condition = BoolNotEquals {
      left: Box::new(true),
      right: Box::new(false),
    };
    assert!(condition.is_met().unwrap());
  }

  #[test]
  fn test_serde() {
    test_init();
    let condition = &BoolNotEquals {
      left: Box::new(true),
      right: Box::new(true),
    } as &dyn Condition;
    let serialized = serde_json::to_string(condition).unwrap();
    assert_eq!(
      serialized,
      r#"{"type":"BoolNotEquals","left":{"type":"Bool","value":true},"right":{"type":"Bool","value":true}}"#
    );
    let deserialized: BoolNotEquals = serde_json::from_str(&serialized).unwrap();
    assert!(!deserialized.is_met().unwrap());
  }
}
