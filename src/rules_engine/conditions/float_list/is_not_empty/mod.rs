use crate::prelude::Context;
use crate::prelude::FloatListValue;
use crate::rules_engine::traits::condition::Condition;
use anyhow::Error as AnyError;
use serde::{Deserialize, Serialize};

#[derive(Derivative, Serialize, Deserialize)]
#[derivative(Debug)]
pub struct FloatListIsNotEmpty {
  #[derivative(Debug = "ignore")]
  pub list: Box<dyn FloatListValue>,
}

#[typetag::serde]
impl Condition for FloatListIsNotEmpty {
  fn is_met(&self, _context: &Box<dyn Context>) -> Result<bool, AnyError> {
    Ok(!self.list.evaluate()?.is_empty())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::prelude::FloatValue;
  use crate::prelude::NullContext;
  use crate::test::init as test_init;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_is_met() {
    test_init();
    let condition = FloatListIsNotEmpty {
      list: Box::<Vec<Box<dyn FloatValue>>>::default(),
    };
    let context = Box::new(NullContext) as Box<dyn Context>;
    assert!(!condition.is_met(&context).unwrap());
  }

  #[test]
  fn test_is_met2() {
    test_init();
    let condition = FloatListIsNotEmpty {
      list: Box::new(vec![Box::new(1_f64) as Box<dyn FloatValue>]),
    };
    let context = Box::new(NullContext) as Box<dyn Context>;
    assert!(condition.is_met(&context).unwrap());
  }

  #[test]
  fn test_serde() {
    test_init();
    let condition = &FloatListIsNotEmpty {
      list: Box::new(vec![Box::new(1_f64) as Box<dyn FloatValue>]),
    } as &dyn Condition;
    let serialized = serde_yaml::to_string(condition).unwrap();
    println!("{}", serialized);
    assert_eq!(
      serialized.trim(),
      r#"
type: FloatListIsNotEmpty
list:
  type: FloatList
  value:
  - type: Float
    value: 1.0
          "#
      .trim()
    );
    let deserialized: FloatListIsNotEmpty = serde_yaml::from_str(&serialized).unwrap();
    let context = Box::new(NullContext) as Box<dyn Context>;
    assert!(deserialized.is_met(&context).unwrap());
  }
}
