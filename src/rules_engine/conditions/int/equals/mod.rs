use crate::prelude::Context;
use crate::prelude::IntValue;
use crate::rules_engine::traits::condition::Condition;
use anyhow::Error as AnyError;
use serde::{Deserialize, Serialize};

#[derive(Derivative, Serialize, Deserialize)]
#[derivative(Debug)]
pub struct IntEquals {
  #[derivative(Debug = "ignore")]
  pub left: Box<dyn IntValue>,
  #[derivative(Debug = "ignore")]
  pub right: Box<dyn IntValue>,
}

#[typetag::serde]
impl Condition for IntEquals {
  fn is_met(&self, _context: &dyn Context) -> Result<bool, AnyError> {
    Ok(self.left.evaluate()? == self.right.evaluate()?)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::prelude::NullContext;
  use crate::test::init as test_init;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_is_met() {
    test_init();
    let condition = IntEquals {
      left: Box::new(1),
      right: Box::new(1),
    };
    let context = &NullContext as &dyn Context;
    assert!(condition.is_met(context).unwrap());
  }

  #[test]
  fn test_is_met2() {
    test_init();
    let condition = IntEquals {
      left: Box::new(1),
      right: Box::new(2),
    };
    let context = &NullContext as &dyn Context;
    assert!(!condition.is_met(context).unwrap());
  }

  #[test]
  fn test_serde() {
    test_init();
    let condition = &IntEquals {
      left: Box::new(1_i64) as Box<dyn IntValue>,
      right: Box::new(1_i64) as Box<dyn IntValue>,
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
    let context = &NullContext as &dyn Context;
    assert!(deserialized.is_met(context).unwrap());
  }
}
