use crate::rules_engine::traits::condition::Condition;
use anyhow::Error as AnyError;
use serde::{Deserialize, Serialize};

#[derive(Derivative, Serialize, Deserialize)]
#[derivative(Debug)]
pub struct BoolNotEquals {
  #[derivative(Debug = "ignore")]
  pub left: Box<dyn Condition>,
  #[derivative(Debug = "ignore")]
  pub right: Box<dyn Condition>,
}

#[typetag::serde]
impl Condition for BoolNotEquals {
  fn is_met(&self) -> Result<bool, AnyError> {
    Ok(self.left.is_met()? != self.right.is_met()?)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::rules_engine::conditions::constants::always::Always;
  use crate::rules_engine::conditions::constants::never::Never;
  use crate::test::init as test_init;

  #[test]
  fn test_is_met() {
    test_init();
    let condition = BoolNotEquals {
      left: Box::new(Always {}),
      right: Box::new(Never {}),
    };
    assert_eq!(condition.is_met().unwrap(), true);
  }

  #[test]
  fn test_is_not_met() {
    test_init();
    let condition = BoolNotEquals {
      left: Box::new(Always {}),
      right: Box::new(Always {}),
    };
    assert_eq!(condition.is_met().unwrap(), false);
  }

  #[test]
  fn test_serde() {
    test_init();
    let condition = &BoolNotEquals {
      left: Box::new(Always {}),
      right: Box::new(Never {}),
    } as &dyn Condition;
    let serialized = serde_json::to_string(condition).unwrap();
    assert_eq!(serialized, r#"{"type":"BoolNotEquals","left":{"type":"Always"},"right":{"type":"Never"}}"#);
    let deserialized: BoolNotEquals = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized.is_met().unwrap(), true);
  }
}
