use crate::prelude::Context;
use crate::prelude::IntListValue;
use crate::rules_engine::traits::condition::Condition;
use anyhow::Error as AnyError;
use serde::{Deserialize, Serialize};

#[derive(Derivative, Serialize, Deserialize)]
#[derivative(Debug)]
pub struct IntListIsNotEmpty {
  #[derivative(Debug = "ignore")]
  pub list: Box<dyn IntListValue>,
}

#[typetag::serde]
impl Condition for IntListIsNotEmpty {
  fn is_met(&self, _context: &Box<dyn Context>) -> Result<bool, AnyError> {
    Ok(!self.list.evaluate()?.is_empty())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::prelude::IntValue;
  use crate::prelude::NullContext;
  use crate::test::init as test_init;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_is_met() {
    test_init();
    let condition = IntListIsNotEmpty {
      list: Box::<Vec<Box<dyn IntValue>>>::default(),
    };
    let context = Box::new(NullContext) as Box<dyn Context>;
    assert!(!condition.is_met(&context).unwrap());
  }

  #[test]
  fn test_is_met2() {
    test_init();
    let condition = IntListIsNotEmpty {
      list: Box::new(vec![Box::new(1_i64) as Box<dyn IntValue>]),
    };
    let context = Box::new(NullContext) as Box<dyn Context>;
    assert!(condition.is_met(&context).unwrap());
  }

  #[test]
  fn test_serde() {
    test_init();
    let condition = &IntListIsNotEmpty {
      list: Box::new(vec![Box::new(1_i64) as Box<dyn IntValue>]),
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
    let context = Box::new(NullContext) as Box<dyn Context>;
    assert!(deserialized.is_met(&context).unwrap());
  }
}
