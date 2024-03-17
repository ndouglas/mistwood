use crate::prelude::Context;
use crate::prelude::FloatValue;
use crate::rules_engine::traits::condition::Condition;
use anyhow::Error as AnyError;
use serde::{Deserialize, Serialize};

#[derive(Derivative, Serialize, Deserialize)]
#[derivative(Debug)]
pub struct FloatLessThanOrEquals {
  #[derivative(Debug = "ignore")]
  pub left: Box<dyn FloatValue>,
  #[derivative(Debug = "ignore")]
  pub right: Box<dyn FloatValue>,
  pub tolerance: f64,
}

#[typetag::serde]
impl Condition for FloatLessThanOrEquals {
  fn is_met(&self, _context: &dyn Context) -> Result<bool, AnyError> {
    let less_than = self.right.evaluate()? - self.left.evaluate()? > self.tolerance;
    let equals = (self.left.evaluate()? - self.right.evaluate()?).abs() < self.tolerance;
    Ok(less_than || equals)
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
    let condition = FloatLessThanOrEquals {
      left: Box::new(1.0),
      right: Box::new(1.0),
      tolerance: 0.001,
    };
    let context = &NullContext as &dyn Context;
    assert!(condition.is_met(context).unwrap());
  }

  #[test]
  fn test_is_met2() {
    test_init();
    let condition = FloatLessThanOrEquals {
      left: Box::new(1.0),
      right: Box::new(1.1),
      tolerance: 0.05,
    };
    let context = &NullContext as &dyn Context;
    assert!(condition.is_met(context).unwrap());
  }

  #[test]
  fn test_is_met3() {
    test_init();
    let condition = FloatLessThanOrEquals {
      left: Box::new(1.1),
      right: Box::new(1.0),
      tolerance: 0.05,
    };
    let context = &NullContext as &dyn Context;
    assert!(!condition.is_met(context).unwrap());
  }

  #[test]
  fn test_serde() {
    test_init();
    let condition = &FloatLessThanOrEquals {
      left: Box::new(1.0),
      right: Box::new(1.0),
      tolerance: 0.001,
    } as &dyn Condition;
    let serialized = serde_yaml::to_string(condition).unwrap();
    println!("{}", serialized);
    assert_eq!(
      serialized.trim(),
      r#"
type: FloatLessThanOrEquals
left:
  type: Float
  value: 1.0
right:
  type: Float
  value: 1.0
tolerance: 0.001
          "#
      .trim()
    );
    let deserialized: FloatLessThanOrEquals = serde_yaml::from_str(&serialized).unwrap();
    let context = &NullContext as &dyn Context;
    assert!(deserialized.is_met(context).unwrap());
  }
}
