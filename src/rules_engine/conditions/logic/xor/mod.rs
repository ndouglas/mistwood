use crate::prelude::Context;
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
  fn is_met(&self, context: &dyn Context) -> Result<bool, AnyError> {
    let mut met = false;
    for condition in &self.conditions {
      if condition.is_met(context)? {
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
  use crate::prelude::NullContext;
  use crate::rules_engine::conditions::constants::always::Always;
  use crate::rules_engine::conditions::constants::error::Error;
  use crate::rules_engine::conditions::constants::never::Never;
  use crate::test::init as test_init;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_is_met() {
    test_init();
    let conditions = vec![Box::new(Always {}) as Box<dyn Condition>];
    let condition = Xor { conditions };
    let context = &NullContext as &dyn Context;
    assert!(condition.is_met(context).unwrap());
  }

  #[test]
  fn test_is_not_met() {
    test_init();
    let conditions = vec![Box::new(Never {}) as Box<dyn Condition>];
    let condition = Xor { conditions };
    let context = &NullContext as &dyn Context;
    assert!(!condition.is_met(context).unwrap());
  }

  #[test]
  fn test_is_error() {
    test_init();
    let conditions = vec![Box::new(Error {
      message: "test".to_string(),
    }) as Box<dyn Condition>];
    let condition = Xor { conditions };
    let context = &NullContext as &dyn Context;
    assert!(condition.is_met(context).is_err());
  }

  #[test]
  fn test_is_met_with_multiple_conditions() {
    test_init();
    let conditions = vec![
      Box::new(Always {}) as Box<dyn Condition>,
      Box::new(Always {}) as Box<dyn Condition>,
    ];
    let condition = Xor { conditions };
    let context = &NullContext as &dyn Context;
    assert!(!condition.is_met(context).unwrap());
  }

  #[test]
  fn test_is_met_with_multiple_conditions2() {
    test_init();
    let conditions = vec![
      Box::new(Always {}) as Box<dyn Condition>,
      Box::new(Never {}) as Box<dyn Condition>,
    ];
    let condition = Xor { conditions };
    let context = &NullContext as &dyn Context;
    assert!(condition.is_met(context).unwrap());
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
    let context = &NullContext as &dyn Context;
    assert!(!condition.is_met(context).unwrap());
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
    let context = &NullContext as &dyn Context;
    assert!(condition.is_met(context).unwrap());
  }

  #[test]
  fn test_is_met_with_multiple_conditions_and_error() {
    test_init();
    let conditions = vec![
      Box::new(Always {}) as Box<dyn Condition>,
      Box::new(Error {
        message: "test".to_string(),
      }) as Box<dyn Condition>,
    ];
    let condition = Xor { conditions };
    let context = &NullContext as &dyn Context;
    assert!(condition.is_met(context).is_err());
  }

  #[test]
  fn test_is_met_with_multiple_conditions_and_error2() {
    test_init();
    let conditions = vec![
      Box::new(Never {}) as Box<dyn Condition>,
      Box::new(Error {
        message: "test".to_string(),
      }) as Box<dyn Condition>,
    ];
    let condition = Xor { conditions };
    let context = &NullContext as &dyn Context;
    assert!(condition.is_met(context).is_err());
  }

  #[test]
  fn test_is_met_with_multiple_conditions_and_error3() {
    test_init();
    let conditions = vec![
      Box::new(Always {}) as Box<dyn Condition>,
      Box::new(Never {}) as Box<dyn Condition>,
      Box::new(Error {
        message: "test".to_string(),
      }) as Box<dyn Condition>,
    ];
    let condition = Xor { conditions };
    let context = &NullContext as &dyn Context;
    assert!(condition.is_met(context).is_err());
  }

  #[test]
  fn test_is_met_with_multiple_conditions_and_error4() {
    test_init();
    let conditions = vec![
      Box::new(Never {}) as Box<dyn Condition>,
      Box::new(Always {}) as Box<dyn Condition>,
      Box::new(Error {
        message: "test".to_string(),
      }) as Box<dyn Condition>,
    ];
    let condition = Xor { conditions };
    let context = &NullContext as &dyn Context;
    assert!(condition.is_met(context).is_err());
  }

  #[test]
  fn test_is_met_with_multiple_conditions_and_error5() {
    test_init();
    let conditions = vec![
      Box::new(Always {}) as Box<dyn Condition>,
      Box::new(Always {}) as Box<dyn Condition>,
      Box::new(Error {
        message: "test".to_string(),
      }) as Box<dyn Condition>,
    ];
    let condition = Xor { conditions };
    let context = &NullContext as &dyn Context;
    assert!(!condition.is_met(context).unwrap());
  }

  #[test]
  fn test_serde() {
    test_init();
    let conditions = vec![
      Box::new(Always {}) as Box<dyn Condition>,
      Box::new(Never {}) as Box<dyn Condition>,
      Box::new(Error {
        message: "test".to_string(),
      }) as Box<dyn Condition>,
    ];
    let condition = &Xor { conditions } as &dyn Condition;
    let serialized = serde_yaml::to_string(condition).unwrap();
    println!("{}", serialized);
    assert_eq!(
      serialized.trim(),
      r#"
type: Xor
conditions:
- type: Always
- type: Never
- type: Error
  message: test
    "#
      .trim()
    );
    let deserialized: Xor = serde_yaml::from_str(&serialized).unwrap();
    let context = &NullContext as &dyn Context;
    assert_eq!(deserialized.conditions.len(), 3);
    assert!(deserialized.conditions[0].is_met(context).unwrap());
    assert!(!deserialized.conditions[1].is_met(context).unwrap());
    assert!(deserialized.conditions[2].is_met(context).is_err());
    assert!(deserialized.is_met(context).is_err());
  }

  #[test]
  fn test_serde_with_multiple_conditions() {
    test_init();
    let conditions = vec![
      Box::new(Always {}) as Box<dyn Condition>,
      Box::new(Never {}) as Box<dyn Condition>,
    ];
    let condition = &Xor { conditions } as &dyn Condition;
    let serialized = serde_yaml::to_string(condition).unwrap();
    println!("{}", serialized);
    assert_eq!(
      serialized.trim(),
      r#"
type: Xor
conditions:
- type: Always
- type: Never
      "#
      .trim()
    );
    let deserialized: Xor = serde_yaml::from_str(&serialized).unwrap();
    let context = &NullContext as &dyn Context;
    assert_eq!(deserialized.conditions.len(), 2);
    assert!(deserialized.conditions[0].is_met(context).unwrap());
    assert!(!deserialized.conditions[1].is_met(context).unwrap());
    assert!(deserialized.is_met(context).unwrap());
  }
}
