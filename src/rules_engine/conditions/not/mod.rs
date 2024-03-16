use crate::rules_engine::traits::condition::Condition;
use anyhow::Error as AnyError;
use serde::{Deserialize, Serialize};

#[derive(Derivative, Serialize, Deserialize)]
#[derivative(Debug)]
pub struct Not {
  #[derivative(Debug = "ignore")]
  pub inner: Box<dyn Condition>,
}

#[typetag::serde]
impl Condition for Not {
  fn is_met(&self) -> Result<bool, AnyError> {
    Ok(!self.inner.is_met()?)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::rules_engine::conditions::always::Always;
  use crate::rules_engine::conditions::error::Error;
  use crate::rules_engine::conditions::never::Never;
  use crate::test::init as test_init;

  #[test]
  fn test_is_met() {
    test_init();
    let inner = Always {};
    let condition = Not { inner: Box::new(inner) };
    assert_eq!(condition.is_met().unwrap(), false);
  }

  #[test]
  fn test_is_not_met() {
    test_init();
    let inner = Never {};
    let condition = Not { inner: Box::new(inner) };
    assert_eq!(condition.is_met().unwrap(), true);
  }

  #[test]
  fn test_is_error() {
    test_init();
    let inner = Error {};
    let condition = Not { inner: Box::new(inner) };
    assert!(condition.is_met().is_err());
  }

  #[test]
  fn test_serde() {
    test_init();
    let inner = Always {};
    let condition = &Not { inner: Box::new(inner) } as &dyn Condition;
    let serialized = serde_json::to_string(condition).unwrap();
    assert_eq!(serialized, r#"{"type":"Not","inner":{"type":"Always"}}"#);
    let deserialized: Box<dyn Condition> = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized.is_met().unwrap(), false);
  }

  #[test]
  fn test_serde_with_error() {
    test_init();
    let inner = Error {};
    let condition = &Not { inner: Box::new(inner) } as &dyn Condition;
    let serialized = serde_json::to_string(condition).unwrap();
    assert_eq!(serialized, r#"{"type":"Not","inner":{"type":"Error"}}"#);
    let deserialized: Box<dyn Condition> = serde_json::from_str(&serialized).unwrap();
    assert!(deserialized.is_met().is_err());
  }
}
