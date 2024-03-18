use crate::prelude::Conclusion;
use crate::prelude::Condition;
use crate::prelude::Context;
use anyhow::Error as AnyError;
use serde::{Deserialize, Serialize};

#[derive(Derivative, Serialize, Deserialize)]
#[derivative(Debug)]
pub struct Rule {
  pub name: String,
  pub description: String,
  #[derivative(Debug = "ignore")]
  pub condition: Box<dyn Condition>,
  #[derivative(Debug = "ignore")]
  pub conclusions: Vec<Box<dyn Conclusion>>,
  #[derivative(Debug = "ignore")]
  pub context: Box<dyn Context>,
}

impl Rule {
  pub fn execute(&self) -> Result<(), AnyError> {
    if self.condition.is_met(&*self.context)? {
      for conclusion in &self.conclusions {
        conclusion.execute(&*self.context)?;
      }
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::prelude::Always;
  use crate::prelude::Conclusion;
  use crate::prelude::Condition;
  use crate::prelude::Error as ErrorCondition;
  use crate::prelude::Never;
  use crate::prelude::NoOp;
  use crate::prelude::NullContext;
  use crate::prelude::ThrowError;
  use crate::test::init as test_init;

  #[test]
  fn test_rule() {
    test_init();
    let condition = Box::new(Always) as Box<dyn Condition>;
    let conclusions = vec![Box::new(NoOp) as Box<dyn Conclusion>];
    let context = Box::new(NullContext);
    let rule = Rule {
      name: "test".to_string(),
      description: "test".to_string(),
      condition,
      conclusions,
      context,
    };
    assert!(rule.execute().is_ok());
  }

  #[test]
  fn test_rule2() {
    test_init();
    let condition = Box::new(Never) as Box<dyn Condition>;
    let conclusions = vec![Box::new(ThrowError {
      message: "test".to_string(),
    }) as Box<dyn Conclusion>];
    let context = Box::new(NullContext);
    let rule = Rule {
      name: "test".to_string(),
      description: "test".to_string(),
      condition,
      conclusions,
      context,
    };
    assert!(rule.execute().is_ok());
  }

  #[test]
  fn test_rule3() {
    test_init();
    let condition = Box::new(Always) as Box<dyn Condition>;
    let conclusions = vec![Box::new(ThrowError {
      message: "test".to_string(),
    }) as Box<dyn Conclusion>];
    let context = Box::new(NullContext);
    let rule = Rule {
      name: "test".to_string(),
      description: "test".to_string(),
      condition,
      conclusions,
      context,
    };
    assert!(rule.execute().is_err());
  }

  #[test]
  fn test_rule4() {
    test_init();
    let condition = Box::new(ErrorCondition {
      message: "test".to_string(),
    }) as Box<dyn Condition>;
    let conclusions = vec![Box::new(NoOp) as Box<dyn Conclusion>];
    let context = Box::new(NullContext);
    let rule = Rule {
      name: "test".to_string(),
      description: "test".to_string(),
      condition,
      conclusions,
      context,
    };
    assert!(rule.execute().is_err());
  }
}
