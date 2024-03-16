use crate::rules_engine::traits::condition::Condition;
use crate::rules_engine::traits::int_argument::IntArgument;
use crate::rules_engine::traits::int_list_argument::IntListArgument;
use anyhow::Error as AnyError;
use serde::{Deserialize, Serialize};

#[derive(Derivative, Serialize, Deserialize)]
#[derivative(Debug)]
pub struct IntListLengthEquals {
  #[derivative(Debug = "ignore")]
  pub list: Box<dyn IntListArgument>,
  #[derivative(Debug = "ignore")]
  pub length: Box<dyn IntArgument>,
}

#[typetag::serde]
impl Condition for IntListLengthEquals {
  fn is_met(&self) -> Result<bool, AnyError> {
    Ok(self.list.value()?.len() == self.length.value()? as usize)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::rules_engine::traits::int_argument::IntArgument;
  use crate::test::init as test_init;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_is_met() {
    test_init();
    let condition = IntListLengthEquals {
      list: Box::new(vec![
        Box::new(1_i64) as Box<dyn IntArgument>,
        Box::new(2_i64) as Box<dyn IntArgument>,
        Box::new(3_i64) as Box<dyn IntArgument>,
      ]),
      length: Box::new(3_i64) as Box<dyn IntArgument>,
    };
    assert!(condition.is_met().unwrap());
  }

  #[test]
  fn test_is_met2() {
    test_init();
    let condition = IntListLengthEquals {
      list: Box::new(vec![Box::new(1_i64) as Box<dyn IntArgument>]),
      length: Box::new(3_i64) as Box<dyn IntArgument>,
    };
    assert!(!condition.is_met().unwrap());
  }

  #[test]
  fn test_serde() {
    test_init();
    let condition = &IntListLengthEquals {
      list: Box::new(vec![Box::new(1_i64) as Box<dyn IntArgument>]),
      length: Box::new(1_i64) as Box<dyn IntArgument>,
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
    assert!(deserialized.is_met().unwrap());
  }
}
