use crate::prelude::Context;
use crate::rules_engine::traits::condition::Condition;
use anyhow::Error as AnyError;
use serde::{Deserialize, Serialize};

#[derive(Derivative, Serialize, Deserialize)]
#[derivative(Debug)]
pub struct Buffer {
  #[derivative(Debug = "ignore")]
  pub inner: Box<dyn Condition>,
}

#[typetag::serde]
impl Condition for Buffer {
  fn is_met(&self, context: &dyn Context) -> Result<bool, AnyError> {
    self.inner.is_met(context)
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

  #[test]
  fn test_is_met() {
    test_init();
    let inner = Always;
    let condition = Buffer { inner: Box::new(inner) };
    let context = &NullContext as &dyn Context;
    assert!(condition.is_met(context).unwrap());
  }

  #[test]
  fn test_is_not_met() {
    test_init();
    let inner = Never {};
    let condition = Buffer { inner: Box::new(inner) };
    let context = &NullContext as &dyn Context;
    assert!(!condition.is_met(context).unwrap());
  }

  #[test]
  fn test_is_error() {
    test_init();
    let inner = Error {
      message: "test".to_string(),
    };
    let condition = Buffer { inner: Box::new(inner) };
    let context = &NullContext as &dyn Context;
    assert!(condition.is_met(context).is_err());
  }

  #[test]
  fn test_serde() {
    test_init();
    let inner = Always;
    let condition = Buffer { inner: Box::new(inner) };
    let serialized = serde_yaml::to_string(&condition).unwrap();
    println!("{}", serialized);
    let deserialized: Buffer = serde_yaml::from_str(&serialized).unwrap();
    let context = &NullContext as &dyn Context;
    assert_eq!(
      condition.is_met(context).unwrap(),
      deserialized.is_met(context).unwrap()
    );
  }

  #[test]
  fn test_serde_with_error() {
    test_init();
    let inner = Error {
      message: "test".to_string(),
    };
    let condition = Buffer { inner: Box::new(inner) };
    let serialized = serde_yaml::to_string(&condition).unwrap();
    println!("{}", serialized);
    let deserialized: Result<Buffer, _> = serde_yaml::from_str(&serialized);
    let context = &NullContext as &dyn Context;
    assert!(deserialized.unwrap().is_met(context).is_err());
  }
}
