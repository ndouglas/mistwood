use crate::rules_engine::traits::float_argument::FloatArgument;
use crate::rules_engine::traits::condition::Condition;
use anyhow::Error as AnyError;
use serde::{Deserialize, Serialize};

#[derive(Derivative, Serialize, Deserialize)]
#[derivative(Debug)]
pub struct FloatGreaterThanOrEquals {
  #[derivative(Debug = "ignore")]
  pub left: Box<dyn FloatArgument>,
  #[derivative(Debug = "ignore")]
  pub right: Box<dyn FloatArgument>,
  pub tolerance: f64,
}

#[typetag::serde]
impl Condition for FloatGreaterThanOrEquals {
  fn is_met(&self) -> Result<bool, AnyError> {
    let greater_than = self.left.value()? - self.right.value()? > self.tolerance;
    let equals = (self.left.value()? - self.right.value()?).abs() < self.tolerance;
    Ok(greater_than || equals)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test::init as test_init;

  #[test]
  fn test_is_met() {
    test_init();
    let condition = FloatGreaterThanOrEquals {
      left: Box::new(1.0),
      right: Box::new(1.0),
      tolerance: 0.001,
    };
    assert_eq!(condition.is_met().unwrap(), true);
  }

  #[test]
  fn test_is_met2() {
    test_init();
    let condition = FloatGreaterThanOrEquals {
      left: Box::new(1.0),
      right: Box::new(1.1),
      tolerance: 0.05,
    };
    assert_eq!(condition.is_met().unwrap(), false);
  }

  #[test]
  fn test_is_met3() {
    test_init();
    let condition = FloatGreaterThanOrEquals {
      left: Box::new(1.1),
      right: Box::new(1.0),
      tolerance: 0.05,
    };
    assert_eq!(condition.is_met().unwrap(), true);
  }

  #[test]
  fn test_serde() {
    test_init();
    let condition = &FloatGreaterThanOrEquals {
      left: Box::new(1.0),
      right: Box::new(1.0),
      tolerance: 0.001,
    } as &dyn Condition;
    let serialized = serde_json::to_string(condition).unwrap();
    assert_eq!(
      serialized,
      r#"{"type":"FloatGreaterThanOrEquals","left":{"type":"Float","value":1.0},"right":{"type":"Float","value":1.0},"tolerance":0.001}"#
    );
    let deserialized: FloatGreaterThanOrEquals = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized.is_met().unwrap(), true);
  }
}
