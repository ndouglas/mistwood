use crate::prelude::Conclusion;
use crate::prelude::Context;
use anyhow::Error as AnyError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ThrowError {
  pub message: String,
}

#[typetag::serde]
impl Conclusion for ThrowError {
  fn execute(&self, _context: &dyn Context) -> Result<(), AnyError> {
    Err(AnyError::msg(self.message.clone()))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::prelude::Conclusion;
  use crate::prelude::Context;
  use crate::prelude::NullContext;
  use crate::test::init as test_init;

  #[test]
  fn test_throw_error() {
    test_init();
    let throw_error = ThrowError {
      message: "test".to_string(),
    };
    let context = &NullContext as &dyn Context;
    assert!(throw_error.execute(context).is_err());
    assert_eq!(throw_error.execute(context).unwrap_err().to_string(), "test");
  }
}
