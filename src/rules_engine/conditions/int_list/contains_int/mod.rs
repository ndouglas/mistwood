use crate::prelude::IntArgument;
use crate::prelude::IntListArgument;
use crate::rules_engine::traits::condition::Condition;
use anyhow::Error as AnyError;
use serde::{Deserialize, Serialize};

#[derive(Derivative, Serialize, Deserialize)]
#[derivative(Debug)]
pub struct IntListContainsInt {
  #[derivative(Debug = "ignore")]
  pub list: Box<dyn IntListArgument>,
  #[derivative(Debug = "ignore")]
  pub value: Box<dyn IntArgument>,
}

#[typetag::serde]
impl Condition for IntListContainsInt {
  fn is_met(&self) -> Result<bool, AnyError> {
    // We have a list of int arguments and we're looking for an int value.
    // So we will map the list of int arguments (evaluating each to get an int value),
    // evaluate the value we're looking for,
    // and then check if the list contains the value we're looking for.
    let list = self.list.evaluate()?;
    let value = self.value.evaluate()?;
    Ok(list.contains(&value))
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
    let condition = IntListContainsInt {
      list: Box::<Vec<Box<dyn IntArgument>>>::default(),
      value: Box::new(1),
    };
    assert!(!condition.is_met().unwrap());
  }

  #[test]
  fn test_is_met2() {
    test_init();
    let condition = IntListContainsInt {
      list: Box::new(vec![Box::new(1_i64) as Box<dyn IntArgument>]),
      value: Box::new(1),
    };
    assert!(condition.is_met().unwrap());
  }

  #[test]
  fn test_serde() {
    test_init();
    let condition = &IntListContainsInt {
      list: Box::new(vec![Box::new(1_i64) as Box<dyn IntArgument>]),
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
