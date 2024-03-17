use crate::prelude::IntArgument;
use crate::prelude::IntRangeArgument;
use crate::rules_engine::traits::condition::Condition;
use anyhow::Error as AnyError;
use serde::{Deserialize, Serialize};

#[derive(Derivative, Serialize, Deserialize)]
#[derivative(Debug)]
pub struct IntInRange {
  #[derivative(Debug = "ignore")]
  pub value: Box<dyn IntArgument>,
  #[derivative(Debug = "ignore")]
  pub range: Box<dyn IntRangeArgument>,
}

#[typetag::serde]
impl Condition for IntInRange {
  fn is_met(&self) -> Result<bool, AnyError> {
    let value = self.value.evaluate()?;
    let range = self.range.evaluate()?;
    Ok(range.contains(&value))
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
    let condition = IntInRange {
      value: Box::new(1),
      range: Box::new(Range { start: 0, end: 2 }),
    };
    assert!(condition.is_met().unwrap());
  }

  #[test]
  fn test_is_met2() {
    test_init();
    let condition = IntInRange {
      value: Box::new(1),
      range: Box::new(Range { start: 1, end: 2 }),
    };
    assert!(condition.is_met().unwrap());
  }

  #[test]
  fn test_is_met3() {
    test_init();
    let condition = IntInRange {
      value: Box::new(2),
      range: Box::new(Range { start: 1, end: 2 }),
    };
    assert!(!condition.is_met().unwrap());
  }

  #[test]
  fn test_serde() {
    test_init();
    let condition = &IntInRange {
      value: Box::new(1),
      range: Box::new(Range { start: 1, end: 2 }),
    } as &dyn Condition;
    let serialized = serde_yaml::to_string(condition).unwrap();
    println!("{}", serialized);
    assert_eq!(
      serialized.trim(),
      r#"
type: IntInRange
value:
  type: Int
  value: 1
range:
  type: IntRange
  start: 1
  end: 2
          "#
      .trim()
    );
    let deserialized: IntInRange = serde_yaml::from_str(&serialized).unwrap();
    assert_eq!(deserialized.value.evaluate().unwrap(), 1);
    assert_eq!(deserialized.range.evaluate().unwrap(), Range { start: 1, end: 2 });
    assert!(deserialized.is_met().unwrap());
  }
}
