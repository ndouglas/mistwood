use crate::prelude::BoolValue;
use crate::rules_engine::traits::condition::Condition;
use anyhow::Error as AnyError;
use serde::{Deserialize, Serialize};

#[derive(Derivative, Serialize, Deserialize)]
#[derivative(Debug)]
pub struct BoolEquals {
  #[derivative(Debug = "ignore")]
  pub left: Box<dyn BoolValue>,
  #[derivative(Debug = "ignore")]
  pub right: Box<dyn BoolValue>,
}

#[typetag::serde]
impl Condition for BoolEquals {
  fn is_met(&self) -> Result<bool, AnyError> {
    Ok(self.left.evaluate()? == self.right.evaluate()?)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test::init as test_init;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_is_met() {
    test_init();
    let condition = BoolEquals {
      left: Box::new(true),
      right: Box::new(true),
    };
    assert!(condition.is_met().unwrap());
  }

  #[test]
  fn test_is_not_met() {
    test_init();
    let condition = BoolEquals {
      left: Box::new(true),
      right: Box::new(false),
    };
    assert!(!condition.is_met().unwrap());
  }

  #[test]
  fn test_serde() {
    test_init();
    let condition = &BoolEquals {
      left: Box::new(true),
      right: Box::new(true),
    } as &dyn Condition;
    let serialized = serde_yaml::to_string(condition).unwrap();
    println!("{}", serialized);
    assert_eq!(
      serialized.trim(),
      r#"
type: BoolEquals
left:
  type: Bool
  value: true
right:
  type: Bool
  value: true
          "#
      .trim()
    );
    let deserialized: BoolEquals = serde_yaml::from_str(&serialized).unwrap();
    assert!(deserialized.is_met().unwrap());
  }
}
