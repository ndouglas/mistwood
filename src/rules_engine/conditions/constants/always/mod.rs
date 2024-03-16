use crate::rules_engine::traits::condition::Condition;
use anyhow::Error as AnyError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Always;

#[typetag::serde]
impl Condition for Always {
  fn is_met(&self) -> Result<bool, AnyError> {
    Ok(true)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test::init as test_init;

  #[test]
  fn test_is_met() {
    test_init();
    let condition = Always;
    assert!(condition.is_met().unwrap());
  }

  #[test]
  fn test_serde() {
    test_init();
    let condition = &Always as &dyn Condition;
    let serialized = serde_json::to_string(condition).unwrap();
    assert_eq!(serialized, r#"{"type":"Always"}"#);
    let deserialized: Box<dyn Condition> = serde_json::from_str(&serialized).unwrap();
    assert!(deserialized.is_met().unwrap());
  }
}
