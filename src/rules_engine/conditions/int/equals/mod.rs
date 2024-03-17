use crate::prelude::IntArgument;
use crate::rules_engine::traits::condition::Condition;
use anyhow::Error as AnyError;
use serde::{Deserialize, Serialize};

#[derive(Derivative, Serialize, Deserialize)]
#[derivative(Debug)]
pub struct IntEquals {
  #[derivative(Debug = "ignore")]
  pub left: Box<dyn IntArgument>,
  #[derivative(Debug = "ignore")]
  pub right: Box<dyn IntArgument>,
}

#[typetag::serde]
impl Condition for IntEquals {
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
    let condition = IntEquals {
      left: Box::new(1),
      right: Box::new(1),
    };
    assert!(condition.is_met().unwrap());
  }

  #[test]
  fn test_is_met2() {
    test_init();
    let condition = IntEquals {
      left: Box::new(1),
      right: Box::new(2),
    };
    assert!(!condition.is_met().unwrap());
  }

  #[test]
  fn test_serde() {
    test_init();
    let condition = &IntEquals {
      left: Box::new(1_i64) as Box<dyn IntArgument>,
      right: Box::new(1_i64) as Box<dyn IntArgument>,
    } as &dyn Condition;
    let serialized = serde_yaml::to_string(condition).unwrap();
    println!("{}", serialized);
    assert_eq!(
      serialized.trim(),
      r#"
type: IntEquals
left:
  type: Int
  value: 1
right:
  type: Int
  value: 1
          "#
      .trim()
    );
    let deserialized: IntEquals = serde_yaml::from_str(&serialized).unwrap();
    assert!(deserialized.is_met().unwrap());
  }
}
