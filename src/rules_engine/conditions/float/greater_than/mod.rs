use crate::rules_engine::traits::condition::Condition;
use crate::rules_engine::traits::float_argument::FloatArgument;
use anyhow::Error as AnyError;
use serde::{Deserialize, Serialize};

#[derive(Derivative, Serialize, Deserialize)]
#[derivative(Debug)]
pub struct FloatGreaterThan {
  #[derivative(Debug = "ignore")]
  pub left: Box<dyn FloatArgument>,
  #[derivative(Debug = "ignore")]
  pub right: Box<dyn FloatArgument>,
  pub tolerance: f64,
}

#[typetag::serde]
impl Condition for FloatGreaterThan {
  fn is_met(&self) -> Result<bool, AnyError> {
    Ok(self.left.value()? - self.right.value()? > self.tolerance)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test::init as test_init;

  #[test]
  fn test_is_met() {
    test_init();
    let condition = FloatGreaterThan {
      left: Box::new(1.0),
      right: Box::new(1.0),
      tolerance: 0.001,
    };
    assert_eq!(condition.is_met().unwrap(), false);
  }

  #[test]
  fn test_is_met2() {
    test_init();
    let condition = FloatGreaterThan {
      left: Box::new(1.0),
      right: Box::new(1.1),
      tolerance: 0.05,
    };
    assert_eq!(condition.is_met().unwrap(), false);
  }

  #[test]
  fn test_is_met3() {
    test_init();
    let condition = FloatGreaterThan {
      left: Box::new(1.1),
      right: Box::new(1.0),
      tolerance: 0.05,
    };
    assert_eq!(condition.is_met().unwrap(), true);
  }

  #[test]
  fn test_serde() {
    test_init();
    let condition = &FloatGreaterThan {
      left: Box::new(1.0),
      right: Box::new(1.0),
      tolerance: 0.001,
    } as &dyn Condition;
    let serialized = serde_json::to_string(condition).unwrap();
    assert_eq!(
      serialized,
      r#"{"type":"FloatGreaterThan","left":{"type":"Float","value":1.0},"right":{"type":"Float","value":1.0},"tolerance":0.001}"#
    );
    let deserialized: FloatGreaterThan = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized.is_met().unwrap(), false);
  }
}
