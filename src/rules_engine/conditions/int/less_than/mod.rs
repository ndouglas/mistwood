use crate::rules_engine::traits::condition::Condition;
use crate::rules_engine::traits::int_argument::IntArgument;
use anyhow::Error as AnyError;
use serde::{Deserialize, Serialize};

#[derive(Derivative, Serialize, Deserialize)]
#[derivative(Debug)]
pub struct IntLessThan {
  #[derivative(Debug = "ignore")]
  pub left: Box<dyn IntArgument>,
  #[derivative(Debug = "ignore")]
  pub right: Box<dyn IntArgument>,
}

#[typetag::serde]
impl Condition for IntLessThan {
  fn is_met(&self) -> Result<bool, AnyError> {
    Ok(self.left.value()? < self.right.value()?)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test::init as test_init;

  #[test]
  fn test_is_met() {
    test_init();
    let condition = IntLessThan {
      left: Box::new(1),
      right: Box::new(1),
    };
    assert_eq!(condition.is_met().unwrap(), false);
  }

  #[test]
  fn test_is_met2() {
    test_init();
    let condition = IntLessThan {
      left: Box::new(1),
      right: Box::new(2),
    };
    assert_eq!(condition.is_met().unwrap(), true);
  }

  #[test]
  fn test_is_met3() {
    test_init();
    let condition = IntLessThan {
      left: Box::new(2),
      right: Box::new(1),
    };
    assert_eq!(condition.is_met().unwrap(), false);
  }

  #[test]
  fn test_serde() {
    test_init();
    let condition = &IntLessThan {
      left: Box::new(1),
      right: Box::new(1),
    } as &dyn Condition;
    let serialized = serde_json::to_string(condition).unwrap();
    assert_eq!(
      serialized,
      r#"{"type":"IntLessThan","left":{"type":"Int","value":1},"right":{"type":"Int","value":1}}"#
    );
    let deserialized: IntLessThan = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized.is_met().unwrap(), false);
  }
}
