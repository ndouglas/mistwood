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
    if self.condition.is_met(&self.context)? {
      for conclusion in &self.conclusions {
        conclusion.execute(&self.context)?;
      }
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::prelude::Conclusion;
  use crate::prelude::Condition;
  use crate::prelude::NullContext;
  use crate::test::init as test_init;

  #[test]
  fn test_rule() {
    test_init();
    let condition = Box::new(crate::prelude::Always) as Box<dyn Condition>;
    let conclusions = vec![Box::new(crate::prelude::NoOp) as Box<dyn Conclusion>];
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
}
