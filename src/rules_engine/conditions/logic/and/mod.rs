use crate::rules_engine::traits::condition::Condition;
use anyhow::Error as AnyError;
use serde::{Deserialize, Serialize};

#[derive(Derivative, Serialize, Deserialize)]
#[derivative(Debug)]
pub struct And {
  #[derivative(Debug = "ignore")]
  pub conditions: Vec<Box<dyn Condition>>,
}

#[typetag::serde]
impl Condition for And {
  fn is_met(&self) -> Result<bool, AnyError> {
    for condition in &self.conditions {
      if !condition.is_met()? {
        return Ok(false);
      }
    }
    Ok(true)
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
    let condition = And { conditions };
    assert_eq!(condition.is_met().unwrap(), true);
  }

  #[test]
  fn test_is_not_met() {
    test_init();
    let conditions = vec![Box::new(Never {}) as Box<dyn Condition>];
    let condition = And { conditions };
    assert_eq!(condition.is_met().unwrap(), false);
  }

  #[test]
  fn test_is_error() {
    test_init();
    let conditions = vec![Box::new(Error {}) as Box<dyn Condition>];
    let condition = And { conditions };
    assert!(condition.is_met().is_err());
  }

  #[test]
  fn test_is_met_with_multiple_conditions() {
    test_init();
    let conditions = vec![
      Box::new(Always {}) as Box<dyn Condition>,
      Box::new(Always {}) as Box<dyn Condition>,
    ];
    let condition = And { conditions };
    assert_eq!(condition.is_met().unwrap(), true);
  }

  #[test]
  fn test_is_not_met_with_multiple_conditions() {
    test_init();
    let conditions = vec![
      Box::new(Always {}) as Box<dyn Condition>,
      Box::new(Never {}) as Box<dyn Condition>,
    ];
    let condition = And { conditions };
    assert_eq!(condition.is_met().unwrap(), false);
  }

  #[test]
  fn test_is_error_with_multiple_conditions() {
    test_init();
    let conditions = vec![
      Box::new(Always {}) as Box<dyn Condition>,
      Box::new(Error {}) as Box<dyn Condition>,
    ];
    let condition = And { conditions };
    assert!(condition.is_met().is_err());
  }

  #[test]
  fn test_serde() {
    test_init();
    let conditions = vec![Box::new(Always {}) as Box<dyn Condition>];
    let condition = &And { conditions } as &dyn Condition;
    let serialized = serde_json::to_string(condition).unwrap();
    assert_eq!(serialized, r#"{"type":"And","conditions":[{"type":"Always"}]}"#);
    let deserialized: And = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized.conditions.len(), 1);
    assert_eq!(deserialized.conditions[0].is_met().unwrap(), true);
    assert_eq!(deserialized.is_met().unwrap(), true);
  }

  #[test]
  fn test_serde_with_multiple_conditions() {
    test_init();
    let conditions = vec![
      Box::new(Always {}) as Box<dyn Condition>,
      Box::new(Never {}) as Box<dyn Condition>,
    ];
    let condition = &And { conditions } as &dyn Condition;
    let serialized = serde_json::to_string(condition).unwrap();
    assert_eq!(
      serialized,
      r#"{"type":"And","conditions":[{"type":"Always"},{"type":"Never"}]}"#
    );
    let deserialized: And = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized.conditions.len(), 2);
    assert_eq!(deserialized.conditions[0].is_met().unwrap(), true);
    assert_eq!(deserialized.conditions[1].is_met().unwrap(), false);
    assert_eq!(deserialized.is_met().unwrap(), false);
  }

  #[test]
  fn test_serde_with_error() {
    test_init();
    let conditions = vec![Box::new(Error {}) as Box<dyn Condition>];
    let condition = &And { conditions } as &dyn Condition;
    let serialized = serde_json::to_string(condition).unwrap();
    assert_eq!(serialized, r#"{"type":"And","conditions":[{"type":"Error"}]}"#);
    let deserialized: And = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized.conditions.len(), 1);
    assert!(deserialized.is_met().is_err());
  }
}
