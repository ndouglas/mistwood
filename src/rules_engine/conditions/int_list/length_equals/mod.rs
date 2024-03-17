use crate::prelude::Context;
use crate::prelude::IntListValue;
use crate::prelude::IntValue;
use crate::rules_engine::traits::condition::Condition;
use anyhow::Error as AnyError;
use serde::{Deserialize, Serialize};

#[derive(Derivative, Serialize, Deserialize)]
#[derivative(Debug)]
pub struct IntListLengthEquals {
  #[derivative(Debug = "ignore")]
  pub list: Box<dyn IntListValue>,
  #[derivative(Debug = "ignore")]
  pub length: Box<dyn IntValue>,
}

#[typetag::serde]
impl Condition for IntListLengthEquals {
  fn is_met(&self, _context: &dyn Context) -> Result<bool, AnyError> {
    Ok(self.list.evaluate()?.len() == self.length.evaluate()? as usize)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::prelude::IntValue;
  use crate::prelude::NullContext;
  use crate::test::init as test_init;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_is_met() {
    test_init();
    let condition = IntListLengthEquals {
      list: Box::new(vec![
        Box::new(1_i64) as Box<dyn IntValue>,
        Box::new(2_i64) as Box<dyn IntValue>,
        Box::new(3_i64) as Box<dyn IntValue>,
      ]),
      length: Box::new(3_i64) as Box<dyn IntValue>,
    };
    let context = &NullContext as &dyn Context;
    assert!(condition.is_met(context).unwrap());
  }

  #[test]
  fn test_is_met2() {
    test_init();
    let condition = IntListLengthEquals {
      list: Box::new(vec![Box::new(1_i64) as Box<dyn IntValue>]),
      length: Box::new(3_i64) as Box<dyn IntValue>,
    };
    let context = &NullContext as &dyn Context;
    assert!(!condition.is_met(context).unwrap());
  }

  #[test]
  fn test_serde() {
    test_init();
    let condition = &IntListLengthEquals {
      list: Box::new(vec![Box::new(1_i64) as Box<dyn IntValue>]),
      length: Box::new(1_i64) as Box<dyn IntValue>,
    } as &dyn Condition;
    let serialized = serde_yaml::to_string(condition).unwrap();
    println!("{}", serialized);
    assert_eq!(
      serialized.trim(),
      r#"
type: IntListLengthEquals
list:
  type: IntList
  value:
  - type: Int
    value: 1
length:
  type: Int
  value: 1
          "#
      .trim()
    );
    let deserialized: IntListLengthEquals = serde_yaml::from_str(&serialized).unwrap();
    let context = &NullContext as &dyn Context;
    assert!(deserialized.is_met(context).unwrap());
  }
}
