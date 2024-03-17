use crate::prelude::IntListArgument;
use crate::rules_engine::traits::condition::Condition;
use anyhow::Error as AnyError;
use serde::{Deserialize, Serialize};

#[derive(Derivative, Serialize, Deserialize)]
#[derivative(Debug)]
pub struct IntListIsNotEmpty {
  #[derivative(Debug = "ignore")]
  pub list: Box<dyn IntListArgument>,
}

#[typetag::serde]
impl Condition for IntListIsNotEmpty {
  fn is_met(&self) -> Result<bool, AnyError> {
    Ok(!self.list.evaluate()?.is_empty())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::prelude::IntArgument;
  use crate::test::init as test_init;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_is_met() {
    test_init();
    let condition = IntListIsNotEmpty {
      list: Box::<Vec<Box<dyn IntArgument>>>::default(),
    };
    assert!(!condition.is_met().unwrap());
  }

  #[test]
  fn test_is_met2() {
    test_init();
    let condition = IntListIsNotEmpty {
      list: Box::new(vec![Box::new(1_i64) as Box<dyn IntArgument>]),
    };
    assert!(condition.is_met().unwrap());
  }

  #[test]
  fn test_serde() {
    test_init();
    let condition = &IntListIsNotEmpty {
      list: Box::new(vec![Box::new(1_i64) as Box<dyn IntArgument>]),
    } as &dyn Condition;
    let serialized = serde_yaml::to_string(condition).unwrap();
    println!("{}", serialized);
    assert_eq!(
      serialized.trim(),
      r#"
type: IntListIsNotEmpty
list:
  type: IntList
  value:
  - type: Int
    value: 1
          "#
      .trim()
    );
    let deserialized: IntListIsNotEmpty = serde_yaml::from_str(&serialized).unwrap();
    assert!(deserialized.is_met().unwrap());
  }
}
