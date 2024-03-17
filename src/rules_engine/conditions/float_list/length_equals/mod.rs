use crate::prelude::Context;
use crate::prelude::FloatListValue;
use crate::prelude::IntValue;
use crate::rules_engine::traits::condition::Condition;
use anyhow::Error as AnyError;
use serde::{Deserialize, Serialize};

#[derive(Derivative, Serialize, Deserialize)]
#[derivative(Debug)]
pub struct FloatListLengthEquals {
  #[derivative(Debug = "ignore")]
  pub list: Box<dyn FloatListValue>,
  #[derivative(Debug = "ignore")]
  pub length: Box<dyn IntValue>,
}

#[typetag::serde]
impl Condition for FloatListLengthEquals {
  fn is_met(&self, _context: &Box<dyn Context>) -> Result<bool, AnyError> {
    Ok(self.list.evaluate()?.len() == self.length.evaluate()? as usize)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::prelude::FloatValue;
  use crate::prelude::IntValue;
  use crate::prelude::NullContext;
  use crate::test::init as test_init;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_is_met() {
    test_init();
    let condition = FloatListLengthEquals {
      list: Box::new(vec![
        Box::new(1_f64) as Box<dyn FloatValue>,
        Box::new(2_f64) as Box<dyn FloatValue>,
        Box::new(3_f64) as Box<dyn FloatValue>,
      ]),
      length: Box::new(3_i64) as Box<dyn IntValue>,
    };
    let context = Box::new(NullContext) as Box<dyn Context>;
    assert!(condition.is_met(&context).unwrap());
  }

  #[test]
  fn test_is_met2() {
    test_init();
    let condition = FloatListLengthEquals {
      list: Box::new(vec![Box::new(1_f64) as Box<dyn FloatValue>]),
      length: Box::new(3_i64) as Box<dyn IntValue>,
    };
    let context = Box::new(NullContext) as Box<dyn Context>;
    assert!(!condition.is_met(&context).unwrap());
  }

  #[test]
  fn test_serde() {
    test_init();
    let condition = &FloatListLengthEquals {
      list: Box::new(vec![Box::new(1_f64) as Box<dyn FloatValue>]),
      length: Box::new(1_i64) as Box<dyn IntValue>,
    } as &dyn Condition;
    let serialized = serde_yaml::to_string(condition).unwrap();
    println!("{}", serialized);
    assert_eq!(
      serialized.trim(),
      r#"
type: FloatListLengthEquals
list:
  type: FloatList
  value:
  - type: Float
    value: 1.0
length:
  type: Int
  value: 1
          "#
      .trim()
    );
    let deserialized: FloatListLengthEquals = serde_yaml::from_str(&serialized).unwrap();
    let context = Box::new(NullContext) as Box<dyn Context>;
    assert!(deserialized.is_met(&context).unwrap());
  }
}
