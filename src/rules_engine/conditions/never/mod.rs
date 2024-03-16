use crate::rules_engine::traits::condition::Condition;
use anyhow::Error as AnyError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Never {}

#[typetag::serde]
impl Condition for Never {
  fn is_met(&self) -> Result<bool, AnyError> {
    Ok(false)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test::init as test_init;

  #[test]
  fn test_is_met() {
    test_init();
    let condition = Never {};
    assert_eq!(condition.is_met().unwrap(), false);
  }
}
