use crate::rules_engine::traits::condition::Condition;
use anyhow::Error as AnyError;
use serde::{Deserialize, Serialize};

#[derive(Derivative, Serialize, Deserialize)]
#[derivative(Debug)]
pub struct Or {
  #[derivative(Debug = "ignore")]
  pub conditions: Vec<Box<dyn Condition>>,
}

#[typetag::serde]
impl Condition for Or {
  fn is_met(&self) -> Result<bool, AnyError> {
    for condition in &self.conditions {
      if condition.is_met()? {
        return Ok(true);
      }
    }
    Ok(false)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::rules_engine::conditions::constants::always::Always;
  use crate::rules_engine::conditions::constants::error::Error;
  use crate::rules_engine::conditions::constants::never::Never;
  use crate::test::init as test_init;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_is_met() {
    test_init();
    let conditions = vec![Box::new(Always {}) as Box<dyn Condition>];
    let condition = Or { conditions };
    assert!(condition.is_met().unwrap());
  }

  #[test]
  fn test_is_not_met() {
    test_init();
    let conditions = vec![Box::new(Never {}) as Box<dyn Condition>];
    let condition = Or { conditions };
    assert!(!condition.is_met().unwrap());
  }

  #[test]
  fn test_is_error() {
    test_init();
    let conditions = vec![Box::new(Error {}) as Box<dyn Condition>];
    let condition = Or { conditions };
    assert!(condition.is_met().is_err());
  }

  #[test]
  fn test_is_met_with_multiple_conditions() {
    test_init();
    let conditions = vec![
      Box::new(Always {}) as Box<dyn Condition>,
      Box::new(Always {}) as Box<dyn Condition>,
    ];
    let condition = Or { conditions };
    assert!(condition.is_met().unwrap());
  }

  #[test]
  fn test_is_not_met_with_multiple_conditions() {
    test_init();
    let conditions = vec![
      Box::new(Always {}) as Box<dyn Condition>,
      Box::new(Never {}) as Box<dyn Condition>,
    ];
    let condition = Or { conditions };
    assert!(condition.is_met().unwrap());
  }

  #[test]
  fn test_is_error_with_multiple_conditions() {
    test_init();
    let conditions = vec![
      Box::new(Always {}) as Box<dyn Condition>,
      Box::new(Error {}) as Box<dyn Condition>,
    ];
    let condition = Or { conditions };
    assert!(condition.is_met().unwrap());
  }

  #[test]
  fn test_is_not_met_with_multiple_conditions_and_error() {
    test_init();
    let conditions = vec![
      Box::new(Never {}) as Box<dyn Condition>,
      Box::new(Error {}) as Box<dyn Condition>,
    ];
    let condition = Or { conditions };
    assert!(condition.is_met().is_err());
  }

  #[test]
  fn test_serde() {
    test_init();
    let conditions = vec![Box::new(Always {}) as Box<dyn Condition>];
    let condition = &Or { conditions } as &dyn Condition;
    let serialized = serde_yaml::to_string(condition).unwrap();
    println!("{}", serialized);
    assert_eq!(
      serialized.trim(),
      r#"
type: Or
conditions:
- type: Always
          "#
      .trim()
    );
    let deserialized: Box<dyn Condition> = serde_yaml::from_str(&serialized).unwrap();
    assert!(deserialized.is_met().unwrap());
  }

  #[test]
  fn test_serde_with_multiple_conditions() {
    test_init();
    let conditions = vec![
      Box::new(Always {}) as Box<dyn Condition>,
      Box::new(Never {}) as Box<dyn Condition>,
    ];
    let condition = &Or { conditions } as &dyn Condition;
    let serialized = serde_yaml::to_string(condition).unwrap();
    println!("{}", serialized);
    assert_eq!(
      serialized.trim(),
      r#"
type: Or
conditions:
- type: Always
- type: Never
          "#
      .trim()
    );
    let deserialized: Box<dyn Condition> = serde_yaml::from_str(&serialized).unwrap();
    assert!(deserialized.is_met().unwrap());
  }
}
