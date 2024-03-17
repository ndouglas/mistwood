use crate::prelude::Condition;
use crate::prelude::Context;
use anyhow::Error as AnyError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Always;

#[typetag::serde]
impl Condition for Always {
  fn is_met(&self, _context: &dyn Context) -> Result<bool, AnyError> {
    Ok(true)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::prelude::NullContext;
  use crate::test::init as test_init;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_is_met() {
    test_init();
    let condition = Always;
    let context = &NullContext as &dyn Context;
    assert!(condition.is_met(context).unwrap());
  }

  #[test]
  fn test_serde() {
    test_init();
    let condition = &Always as &dyn Condition;
    let serialized = serde_yaml::to_string(condition).unwrap();
    println!("{}", serialized);
    assert_eq!(
      serialized.trim(),
      r#"
type: Always
          "#
      .trim()
    );
    let deserialized: Box<dyn Condition> = serde_yaml::from_str(&serialized).unwrap();
    let context = &NullContext as &dyn Context;
    assert!(deserialized.is_met(context).unwrap());
  }
}
