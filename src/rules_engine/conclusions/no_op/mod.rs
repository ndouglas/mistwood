use crate::prelude::Conclusion;
use crate::prelude::Context;
use anyhow::Error as AnyError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct NoOp;

#[typetag::serde]
impl Conclusion for NoOp {
  fn execute(&self, _context: &Box<dyn Context>) -> Result<(), AnyError> {
    Ok(())
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
  fn test_no_op() {
    test_init();
    let no_op = NoOp;
    let context = Box::new(NullContext) as Box<dyn Context>;
    assert!(no_op.execute(&context).is_ok());
  }
}
