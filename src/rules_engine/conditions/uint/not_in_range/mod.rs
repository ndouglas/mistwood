use crate::rules_engine::traits::condition::Condition;
use crate::rules_engine::traits::int_argument::IntArgument;
use crate::rules_engine::traits::int_range_argument::IntRangeArgument;
use anyhow::Error as AnyError;
use serde::{Deserialize, Serialize};

#[derive(Derivative, Serialize, Deserialize)]
#[derivative(Debug)]
pub struct UintNotInRange {
  #[derivative(Debug = "ignore")]
  pub value: Box<dyn IntArgument>,
  #[derivative(Debug = "ignore")]
  pub range: Box<dyn IntRangeArgument>,
}

#[typetag::serde]
impl Condition for UintNotInRange {
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

  #[test]
  fn test_is_met() {
    test_init();
    let condition = UintNotInRange {
      value: Box::new(1),
      range: Box::new(Range { start: 0, end: 2 }),
    };
    assert_eq!(condition.is_met().unwrap(), false);
  }

  #[test]
  fn test_is_met2() {
    test_init();
    let condition = UintNotInRange {
      value: Box::new(1),
      range: Box::new(Range { start: 1, end: 2 }),
    };
    assert_eq!(condition.is_met().unwrap(), false);
  }

  #[test]
  fn test_is_met3() {
    test_init();
    let condition = UintNotInRange {
      value: Box::new(2),
      range: Box::new(Range { start: 1, end: 2 }),
    };
    assert_eq!(condition.is_met().unwrap(), true);
  }

  #[test]
  fn test_serde() {
    test_init();
    let condition = &UintNotInRange {
      value: Box::new(1),
      range: Box::new(Range { start: 1, end: 2 }),
    } as &dyn Condition;
    let serialized = serde_json::to_string(condition).unwrap();
    assert_eq!(
      serialized,
      r#"{"type":"UintNotInRange","value":{"type":"Int","value":1},"range":{"type":"Range","start":1,"end":2}}"#
    );
    let deserialized: UintNotInRange = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized.value.value().unwrap(), 1);
    assert_eq!(deserialized.range.value().unwrap(), &Range { start: 1, end: 2 });
    assert_eq!(deserialized.is_met().unwrap(), false);
  }
}
