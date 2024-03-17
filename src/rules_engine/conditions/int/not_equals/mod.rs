use crate::prelude::Context;
use crate::prelude::IntValue;
use crate::rules_engine::traits::condition::Condition;
use anyhow::Error as AnyError;
use serde::{Deserialize, Serialize};

#[derive(Derivative, Serialize, Deserialize)]
#[derivative(Debug)]
pub struct IntNotEquals {
  #[derivative(Debug = "ignore")]
  pub left: Box<dyn IntValue>,
  #[derivative(Debug = "ignore")]
  pub right: Box<dyn IntValue>,
}

#[typetag::serde]
impl Condition for IntNotEquals {
  fn is_met(&self, _context: &Box<dyn Context>) -> Result<bool, AnyError> {
    Ok(self.left.evaluate()? != self.right.evaluate()?)
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
    let condition = IntNotEquals {
      left: Box::new(1),
      right: Box::new(1),
    };
    let context = Box::new(NullContext) as Box<dyn Context>;
    assert!(!condition.is_met(&context).unwrap());
  }

  #[test]
  fn test_is_met2() {
    test_init();
    let condition = IntNotEquals {
      left: Box::new(1),
      right: Box::new(2),
    };
    let context = Box::new(NullContext) as Box<dyn Context>;
    assert!(condition.is_met(&context).unwrap());
  }

  #[test]
  fn test_serde() {
    test_init();
    let condition = &IntNotEquals {
      left: Box::new(1),
      right: Box::new(1),
    } as &dyn Condition;
    let serialized = serde_yaml::to_string(condition).unwrap();
    println!("{}", serialized);
    assert_eq!(
      serialized.trim(),
      r#"
type: IntNotEquals
left:
  type: Int
  value: 1
right:
  type: Int
  value: 1
          "#
      .trim()
    );
    let deserialized: IntNotEquals = serde_yaml::from_str(&serialized).unwrap();
    let context = Box::new(NullContext) as Box<dyn Context>;
    assert!(!deserialized.is_met(&context).unwrap());
  }
}
