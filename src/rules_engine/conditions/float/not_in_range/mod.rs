use crate::rules_engine::traits::condition::Condition;
use crate::rules_engine::traits::float_argument::FloatArgument;
use crate::rules_engine::traits::float_range_argument::FloatRangeArgument;
use anyhow::Error as AnyError;
use serde::{Deserialize, Serialize};

#[derive(Derivative, Serialize, Deserialize)]
#[derivative(Debug)]
pub struct FloatNotInRange {
  #[derivative(Debug = "ignore")]
  pub value: Box<dyn FloatArgument>,
  #[derivative(Debug = "ignore")]
  pub range: Box<dyn FloatRangeArgument>,
}

#[typetag::serde]
impl Condition for FloatNotInRange {
  fn is_met(&self) -> Result<bool, AnyError> {
    let value = self.value.value()?;
    let range = self.range.value()?;
    Ok(!range.contains(&value))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test::init as test_init;
  use core::ops::Range;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_is_met() {
    test_init();
    let condition = FloatNotInRange {
      value: Box::new(1.0),
      range: Box::new(Range { start: 0.0, end: 2.0 }),
    };
    assert!(!condition.is_met().unwrap());
  }

  #[test]
  fn test_is_met2() {
    test_init();
    let condition = FloatNotInRange {
      value: Box::new(1.0),
      range: Box::new(Range { start: 1.0, end: 2.0 }),
    };
    assert!(!condition.is_met().unwrap());
  }

  #[test]
  fn test_is_met3() {
    test_init();
    let condition = FloatNotInRange {
      value: Box::new(1.0),
      range: Box::new(Range { start: 0.0, end: 1.0 }),
    };
    assert!(condition.is_met().unwrap());
  }

  #[test]
  fn test_serde() {
    test_init();
    let condition = &FloatNotInRange {
      value: Box::new(1.0),
      range: Box::new(Range { start: 0.0, end: 1.0 }),
    } as &dyn Condition;
    let serialized = serde_yaml::to_string(condition).unwrap();
    println!("{}", serialized);
    assert_eq!(
      serialized.trim(),
      r#"
type: FloatNotInRange
value:
  type: Float
  value: 1.0
range:
  type: FloatRange
  start: 0.0
  end: 1.0
          "#
      .trim()
    );
    let deserialized: FloatNotInRange = serde_yaml::from_str(&serialized).unwrap();
    assert_eq!(deserialized.value.value().unwrap(), 1.0);
    assert_eq!(deserialized.range.value().unwrap(), &Range { start: 0.0, end: 1.0 });
  }
}
