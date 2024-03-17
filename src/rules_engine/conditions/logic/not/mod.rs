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
  fn is_met(&self, _context: &Box<dyn Context>) -> Result<bool, AnyError> {
    Ok(!self.inner.is_met(&context)?)
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
    let inner = Always {};
    let condition = Not { inner: Box::new(inner) };
    assert_eq!(condition.is_met(&context).unwrap(), false);
  }

  #[test]
  fn test_is_not_met() {
    test_init();
    let inner = Never {};
    let condition = Not { inner: Box::new(inner) };
    assert_eq!(condition.is_met(&context).unwrap(), true);
  }

  #[test]
  fn test_is_error() {
    test_init();
    let inner = Error {};
    let condition = Not { inner: Box::new(inner) };
    assert!(condition.is_met(&context).is_err());
  }

  #[test]
  fn test_serde() {
    test_init();
    let inner = Always {};
    let condition = &Not { inner: Box::new(inner) } as &dyn Condition;
    let serialized = serde_yaml::to_string(condition).unwrap();
    assert_eq!(serialized, r#"{"type":"Not","inner":{"type":"Always"}}"#);
    let deserialized: Box<dyn Condition> = serde_yaml::from_str(&serialized).unwrap();
    assert_eq!(deserialized.is_met(&context).unwrap(), false);
  }

  #[test]
  fn test_serde_with_error() {
    test_init();
    let inner = Error {};
    let condition = &Not { inner: Box::new(inner) } as &dyn Condition;
    let serialized = serde_yaml::to_string(condition).unwrap();
    assert_eq!(serialized, r#"{"type":"Not","inner":{"type":"Error"}}"#);
    let deserialized: Box<dyn Condition> = serde_yaml::from_str(&serialized).unwrap();
    assert!(deserialized.is_met(&context).is_err());
  }
}
