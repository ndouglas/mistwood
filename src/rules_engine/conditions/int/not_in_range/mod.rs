use crate::prelude::Context;
use crate::prelude::IntRangeValue;
use crate::prelude::IntValue;
use crate::rules_engine::traits::condition::Condition;
use anyhow::Error as AnyError;
use serde::{Deserialize, Serialize};

#[derive(Derivative, Serialize, Deserialize)]
#[derivative(Debug)]
pub struct IntNotInRange {
  #[derivative(Debug = "ignore")]
  pub value: Box<dyn IntValue>,
  #[derivative(Debug = "ignore")]
  pub range: Box<dyn IntRangeValue>,
}

#[typetag::serde]
impl Condition for IntNotInRange {
  fn is_met(&self, _context: &dyn Context) -> Result<bool, AnyError> {
    let value = self.value.evaluate()?;
    let range = self.range.evaluate()?;
    Ok(!range.contains(&value))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::prelude::NullContext;
  use crate::test::init as test_init;
  use core::ops::Range;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_is_met() {
    test_init();
    let condition = IntNotInRange {
      value: Box::new(1),
      range: Box::new(Range { start: 0, end: 2 }),
    };
    let context = &NullContext as &dyn Context;
    assert!(!condition.is_met(context).unwrap());
  }

  #[test]
  fn test_is_met2() {
    test_init();
    let condition = IntNotInRange {
      value: Box::new(1),
      range: Box::new(Range { start: 1, end: 2 }),
    };
    let context = &NullContext as &dyn Context;
    assert!(!condition.is_met(context).unwrap());
  }

  #[test]
  fn test_is_met3() {
    test_init();
    let condition = IntNotInRange {
      value: Box::new(2),
      range: Box::new(Range { start: 1, end: 2 }),
    };
    let context = &NullContext as &dyn Context;
    assert!(condition.is_met(context).unwrap());
  }

  #[test]
  fn test_serde() {
    test_init();
    let condition = &IntNotInRange {
      value: Box::new(1),
      range: Box::new(Range { start: 1, end: 2 }),
    } as &dyn Condition;
    let serialized = serde_yaml::to_string(condition).unwrap();
    println!("{}", serialized);
    assert_eq!(
      serialized.trim(),
      r#"
type: IntNotInRange
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
    let deserialized: IntNotInRange = serde_yaml::from_str(&serialized).unwrap();
    assert_eq!(deserialized.value.evaluate().unwrap(), 1);
    assert_eq!(deserialized.range.evaluate().unwrap(), Range { start: 1, end: 2 });
    let context = &NullContext as &dyn Context;
    assert!(!deserialized.is_met(context).unwrap());
  }
}
