use crate::prelude::IntListValue;
use crate::prelude::IntValue;
use crate::rules_engine::traits::condition::Condition;
use anyhow::Error as AnyError;
use serde::{Deserialize, Serialize};

#[derive(Derivative, Serialize, Deserialize)]
#[derivative(Debug)]
pub struct IntListContainsInt {
  #[derivative(Debug = "ignore")]
  pub list: Box<dyn IntListValue>,
  #[derivative(Debug = "ignore")]
  pub value: Box<dyn IntValue>,
}

#[typetag::serde]
impl Condition for IntListContainsInt {
  fn is_met(&self) -> Result<bool, AnyError> {
    let list = self.list.evaluate()?;
    let value = self.value.evaluate()?;
    Ok(list.contains(&value))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::prelude::IntValue;
  use crate::test::init as test_init;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_is_met() {
    test_init();
    let condition = IntListContainsInt {
      list: Box::<Vec<Box<dyn IntValue>>>::default(),
      value: Box::new(1),
    };
    assert!(!condition.is_met().unwrap());
  }

  #[test]
  fn test_is_met2() {
    test_init();
    let condition = IntListContainsInt {
      list: Box::new(vec![Box::new(1_i64) as Box<dyn IntValue>]),
      value: Box::new(1),
    };
    assert!(condition.is_met().unwrap());
  }

  #[test]
  fn test_serde() {
    test_init();
    let condition = &IntListContainsInt {
      list: Box::new(vec![Box::new(1_i64) as Box<dyn IntValue>]),
      value: Box::new(1),
    } as &dyn Condition;
    let serialized = serde_yaml::to_string(condition).unwrap();
    println!("{}", serialized);
    assert_eq!(
      serialized.trim(),
      r#"
type: IntListContainsInt
list:
  type: IntList
  value:
  - type: Int
    value: 1
value:
  type: Int
  value: 1
          "#
      .trim()
    );
    let deserialized: IntListContainsInt = serde_yaml::from_str(&serialized).unwrap();
    assert!(deserialized.is_met().unwrap());
  }
}