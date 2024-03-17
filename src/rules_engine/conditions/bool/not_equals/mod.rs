use crate::prelude::BoolValue;
use crate::prelude::Context;
use crate::rules_engine::traits::condition::Condition;
use anyhow::Error as AnyError;
use serde::{Deserialize, Serialize};

#[derive(Derivative, Serialize, Deserialize)]
#[derivative(Debug)]
pub struct BoolNotEquals {
  #[derivative(Debug = "ignore")]
  pub left: Box<dyn BoolValue>,
  #[derivative(Debug = "ignore")]
  pub right: Box<dyn BoolValue>,
}

#[typetag::serde]
impl Condition for BoolNotEquals {
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
    let condition = BoolNotEquals {
      left: Box::new(true),
      right: Box::new(true),
    };
    let context = Box::new(NullContext) as Box<dyn Context>;
    assert!(!condition.is_met(&context).unwrap());
  }

  #[test]
  fn test_is_met2() {
    test_init();
    let condition = BoolNotEquals {
      left: Box::new(true),
      right: Box::new(false),
    };
    let context = Box::new(NullContext) as Box<dyn Context>;
    assert!(condition.is_met(&context).unwrap());
  }

  #[test]
  fn test_serde() {
    test_init();
    let condition = &BoolNotEquals {
      left: Box::new(true),
      right: Box::new(true),
    } as &dyn Condition;
    let serialized = serde_yaml::to_string(condition).unwrap();
    println!("{}", serialized);
    assert_eq!(
      serialized.trim(),
      r#"
type: BoolNotEquals
left:
  type: Bool
  value: true
right:
  type: Bool
  value: true
          "#
      .trim()
    );
    let deserialized: BoolNotEquals = serde_yaml::from_str(&serialized).unwrap();
    let context = Box::new(NullContext) as Box<dyn Context>;
    assert!(!deserialized.is_met(&context).unwrap());
  }
}
