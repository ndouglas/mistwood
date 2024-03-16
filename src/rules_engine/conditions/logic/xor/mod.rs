use crate::rules_engine::traits::condition::Condition;
use anyhow::Error as AnyError;
use serde::{Deserialize, Serialize};

#[derive(Derivative, Serialize, Deserialize)]
#[derivative(Debug)]
pub struct Xor {
  #[derivative(Debug = "ignore")]
  pub conditions: Vec<Box<dyn Condition>>,
}

#[typetag::serde]
impl Condition for Xor {
  fn is_met(&self) -> Result<bool, AnyError> {
    let mut met = false;
    for condition in &self.conditions {
      if condition.is_met()? {
        if met {
          return Ok(false);
        }
        met = true;
      }
    }
    Ok(met)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::rules_engine::conditions::constants::always::Always;
  use crate::rules_engine::conditions::constants::error::Error;
  use crate::rules_engine::conditions::constants::never::Never;
  use crate::test::init as test_init;

  #[test]
  fn test_is_met() {
    test_init();
    let conditions = vec![Box::new(Always {}) as Box<dyn Condition>];
    let condition = Xor { conditions };
    assert!(condition.is_met().unwrap());
  }

  #[test]
  fn test_is_not_met() {
    test_init();
    let conditions = vec![Box::new(Never {}) as Box<dyn Condition>];
    let condition = Xor { conditions };
    assert!(!condition.is_met().unwrap());
  }

  #[test]
  fn test_is_error() {
    test_init();
    let conditions = vec![Box::new(Error {}) as Box<dyn Condition>];
    let condition = Xor { conditions };
    assert!(condition.is_met().is_err());
  }

  #[test]
  fn test_is_met_with_multiple_conditions() {
    test_init();
    let conditions = vec![
      Box::new(Always {}) as Box<dyn Condition>,
      Box::new(Always {}) as Box<dyn Condition>,
    ];
    let condition = Xor { conditions };
    assert!(!condition.is_met().unwrap());
  }

  #[test]
  fn test_is_met_with_multiple_conditions2() {
    test_init();
    let conditions = vec![
      Box::new(Always {}) as Box<dyn Condition>,
      Box::new(Never {}) as Box<dyn Condition>,
    ];
    let condition = Xor { conditions };
    assert!(condition.is_met().unwrap());
  }

  #[test]
  fn test_is_met_with_multiple_conditions3() {
    test_init();
    let conditions = vec![
      Box::new(Always {}) as Box<dyn Condition>,
      Box::new(Never {}) as Box<dyn Condition>,
      Box::new(Always {}) as Box<dyn Condition>,
    ];
    let condition = Xor { conditions };
    assert!(!condition.is_met().unwrap());
  }

  #[test]
  fn test_is_met_with_multiple_conditions4() {
    test_init();
    let conditions = vec![
      Box::new(Always {}) as Box<dyn Condition>,
      Box::new(Never {}) as Box<dyn Condition>,
      Box::new(Never {}) as Box<dyn Condition>,
    ];
    let condition = Xor { conditions };
    assert!(condition.is_met().unwrap());
  }

  #[test]
  fn test_is_met_with_multiple_conditions_and_error() {
    test_init();
    let conditions = vec![
      Box::new(Always {}) as Box<dyn Condition>,
      Box::new(Error {}) as Box<dyn Condition>,
    ];
    let condition = Xor { conditions };
    assert!(condition.is_met().is_err());
  }

  #[test]
  fn test_is_met_with_multiple_conditions_and_error2() {
    test_init();
    let conditions = vec![
      Box::new(Never {}) as Box<dyn Condition>,
      Box::new(Error {}) as Box<dyn Condition>,
    ];
    let condition = Xor { conditions };
    assert!(condition.is_met().is_err());
  }

  #[test]
  fn test_is_met_with_multiple_conditions_and_error3() {
    test_init();
    let conditions = vec![
      Box::new(Always {}) as Box<dyn Condition>,
      Box::new(Never {}) as Box<dyn Condition>,
      Box::new(Error {}) as Box<dyn Condition>,
    ];
    let condition = Xor { conditions };
    assert!(condition.is_met().is_err());
  }

  #[test]
  fn test_is_met_with_multiple_conditions_and_error4() {
    test_init();
    let conditions = vec![
      Box::new(Never {}) as Box<dyn Condition>,
      Box::new(Always {}) as Box<dyn Condition>,
      Box::new(Error {}) as Box<dyn Condition>,
    ];
    let condition = Xor { conditions };
    assert!(condition.is_met().is_err());
  }

  #[test]
  fn test_is_met_with_multiple_conditions_and_error5() {
    test_init();
    let conditions = vec![
      Box::new(Always {}) as Box<dyn Condition>,
      Box::new(Always {}) as Box<dyn Condition>,
      Box::new(Error {}) as Box<dyn Condition>,
    ];
    let condition = Xor { conditions };
    assert!(!condition.is_met().unwrap());
  }

  #[test]
  fn test_serde() {
    test_init();
    let conditions = vec![
      Box::new(Always {}) as Box<dyn Condition>,
      Box::new(Never {}) as Box<dyn Condition>,
      Box::new(Error {}) as Box<dyn Condition>,
    ];
    let condition = &Xor { conditions } as &dyn Condition;
    let serialized = serde_json::to_string(condition).unwrap();
    assert_eq!(
      serialized,
      r#"{"type":"Xor","conditions":[{"type":"Always"},{"type":"Never"},{"type":"Error"}]}"#
    );
    let deserialized: Xor = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized.conditions.len(), 3);
    assert!(deserialized.conditions[0].is_met().unwrap());
    assert!(!deserialized.conditions[1].is_met().unwrap());
    assert!(deserialized.conditions[2].is_met().is_err());
    assert!(deserialized.is_met().is_err());
  }

  #[test]
  fn test_serde_with_multiple_conditions() {
    test_init();
    let conditions = vec![
      Box::new(Always {}) as Box<dyn Condition>,
      Box::new(Never {}) as Box<dyn Condition>,
    ];
    let condition = &Xor { conditions } as &dyn Condition;
    let serialized = serde_json::to_string(condition).unwrap();
    assert_eq!(
      serialized,
      r#"{"type":"Xor","conditions":[{"type":"Always"},{"type":"Never"}]}"#
    );
    let deserialized: Xor = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized.conditions.len(), 2);
    assert!(deserialized.conditions[0].is_met().unwrap());
    assert!(!deserialized.conditions[1].is_met().unwrap());
    assert!(deserialized.is_met().unwrap());
  }
}
