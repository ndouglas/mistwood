use crate::prelude::Context;
use crate::rules_engine::traits::condition::Condition;
use anyhow::Error as AnyError;
use serde::{Deserialize, Serialize};

#[derive(Derivative, Serialize, Deserialize)]
#[derivative(Debug)]
pub struct Xnor {
  #[derivative(Debug = "ignore")]
  pub conditions: Vec<Box<dyn Condition>>,
}

#[typetag::serde]
impl Condition for Xnor {
  fn is_met(&self, context: &dyn Context) -> Result<bool, AnyError> {
    let mut met = false;
    for condition in &self.conditions {
      if condition.is_met(context)? {
        if met {
          return Ok(true);
        }
        met = true;
      }
    }
    Ok(!met)
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
    let condition = Xnor { conditions };
    let context = &NullContext as &dyn Context;
    assert!(!condition.is_met(context).unwrap());
  }

  #[test]
  fn test_is_not_met() {
    test_init();
    let conditions = vec![Box::new(Never {}) as Box<dyn Condition>];
    let condition = Xnor { conditions };
    let context = &NullContext as &dyn Context;
    assert!(condition.is_met(context).unwrap());
  }

  #[test]
  fn test_is_error() {
    test_init();
    let conditions = vec![Box::new(Error {}) as Box<dyn Condition>];
    let condition = Xnor { conditions };
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
    let condition = Xnor { conditions };
    let context = &NullContext as &dyn Context;
    assert!(condition.is_met(context).unwrap());
  }

  #[test]
  fn test_is_met_with_multiple_conditions2() {
    test_init();
    let conditions = vec![
      Box::new(Always {}) as Box<dyn Condition>,
      Box::new(Never {}) as Box<dyn Condition>,
    ];
    let condition = Xnor { conditions };
    let context = &NullContext as &dyn Context;
    assert!(!condition.is_met(context).unwrap());
  }

  #[test]
  fn test_is_met_with_multiple_conditions3() {
    test_init();
    let conditions = vec![
      Box::new(Always {}) as Box<dyn Condition>,
      Box::new(Never {}) as Box<dyn Condition>,
      Box::new(Always {}) as Box<dyn Condition>,
    ];
    let condition = Xnor { conditions };
    let context = &NullContext as &dyn Context;
    assert!(condition.is_met(context).unwrap());
  }

  #[test]
  fn test_is_met_with_multiple_conditions4() {
    test_init();
    let conditions = vec![
      Box::new(Always {}) as Box<dyn Condition>,
      Box::new(Never {}) as Box<dyn Condition>,
      Box::new(Never {}) as Box<dyn Condition>,
    ];
    let condition = Xnor { conditions };
    let context = &NullContext as &dyn Context;
    assert!(!condition.is_met(context).unwrap());
  }

  #[test]
  fn test_is_met_with_multiple_conditions_and_error() {
    test_init();
    let conditions = vec![
      Box::new(Always {}) as Box<dyn Condition>,
      Box::new(Error {}) as Box<dyn Condition>,
    ];
    let condition = Xnor { conditions };
    let context = &NullContext as &dyn Context;
    assert!(condition.is_met(context).is_err());
  }

  #[test]
  fn test_is_met_with_multiple_conditions_and_error2() {
    test_init();
    let conditions = vec![
      Box::new(Never {}) as Box<dyn Condition>,
      Box::new(Error {}) as Box<dyn Condition>,
    ];
    let condition = Xnor { conditions };
    let context = &NullContext as &dyn Context;
    assert!(condition.is_met(context).is_err());
  }

  #[test]
  fn test_is_met_with_multiple_conditions_and_error3() {
    test_init();
    let conditions = vec![
      Box::new(Always {}) as Box<dyn Condition>,
      Box::new(Never {}) as Box<dyn Condition>,
      Box::new(Error {}) as Box<dyn Condition>,
    ];
    let condition = Xnor { conditions };
    let context = &NullContext as &dyn Context;
    assert!(condition.is_met(context).is_err());
  }

  #[test]
  fn test_is_met_with_multiple_conditions_and_error4() {
    test_init();
    let conditions = vec![
      Box::new(Never {}) as Box<dyn Condition>,
      Box::new(Always {}) as Box<dyn Condition>,
      Box::new(Error {}) as Box<dyn Condition>,
    ];
    let condition = Xnor { conditions };
    let context = &NullContext as &dyn Context;
    assert!(condition.is_met(context).is_err());
  }

  #[test]
  fn test_is_met_with_multiple_conditions_and_error5() {
    test_init();
    let conditions = vec![
      Box::new(Always {}) as Box<dyn Condition>,
      Box::new(Always {}) as Box<dyn Condition>,
      Box::new(Error {}) as Box<dyn Condition>,
    ];
    let condition = Xnor { conditions };
    let context = &NullContext as &dyn Context;
    assert!(condition.is_met(context).unwrap());
  }

  #[test]
  fn test_serde() {
    test_init();
    let conditions = vec![Box::new(Always {}) as Box<dyn Condition>];
    let condition = &Xnor { conditions } as &dyn Condition;
    let serialized = serde_yaml::to_string(condition).unwrap();
    println!("{}", serialized);
    assert_eq!(
      serialized.trim(),
      r#"
type: Xnor
conditions:
- type: Always
          "#
      .trim()
    );
    let deserialized: Xnor = serde_yaml::from_str(&serialized).unwrap();
    let context = &NullContext as &dyn Context;
    assert_eq!(deserialized.conditions.len(), 1);
    assert!(deserialized.conditions[0].is_met(context).unwrap());
    assert!(!deserialized.is_met(context).unwrap());
  }
}
