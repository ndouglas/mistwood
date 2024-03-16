use crate::rules_engine::traits::condition::Condition;
use crate::rules_engine::traits::int_list_argument::IntListArgument;
use anyhow::Error as AnyError;
use serde::{Deserialize, Serialize};

#[derive(Derivative, Serialize, Deserialize)]
#[derivative(Debug)]
pub struct IntListIsNotEmpty {
  #[derivative(Debug = "ignore")]
  pub list: Box<dyn IntListArgument>,
}

#[typetag::serde]
impl Condition for IntListIsNotEmpty {
  fn is_met(&self) -> Result<bool, AnyError> {
    Ok(!self.list.value()?.is_empty())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::rules_engine::traits::int_argument::IntArgument;
  use crate::test::init as test_init;

  #[test]
  fn test_is_met() {
    test_init();
    let condition = IntListIsNotEmpty { list: Box::new(vec![]) };
    assert_eq!(condition.is_met().unwrap(), false);
  }

  #[test]
  fn test_is_met2() {
    test_init();
    let condition = IntListIsNotEmpty {
      list: Box::new(vec![Box::new(1 as i64) as Box<dyn IntArgument>]),
    };
    assert_eq!(condition.is_met().unwrap(), true);
  }

  #[test]
  fn test_serde() {
    test_init();
    let condition = &IntListIsNotEmpty {
      list: Box::new(vec![Box::new(1 as i64) as Box<dyn IntArgument>]),
    } as &dyn Condition;
    let serialized = serde_json::to_string(condition).unwrap();
    assert_eq!(
      serialized,
      r#"{"type":"IntListIsNotEmpty","list":{"type":"IntList","value":[{"type":"Int","value":1}]}}"#
    );
    let deserialized: IntListIsNotEmpty = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized.is_met().unwrap(), true);
  }
}
