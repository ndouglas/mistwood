use crate::commands::prelude::Command;
use crate::input::_errors::input::InputError;
use crate::input::input::Input;

/// A trait for input handlers.
pub trait Handler {
  /// Read input from the handler.
  fn read(&mut self, input: &Input) -> Result<Box<dyn Command>, InputError>;
}
