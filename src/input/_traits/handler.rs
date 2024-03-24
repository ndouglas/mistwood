use crate::commands::prelude::Command;
use crate::input::_errors::input::InputError;
use crate::input::input::Input;

/// A trait for input handlers.
///
/// An input handler reads input from the user and uses some process to derive
/// a command from it.
pub trait Handler {
  /// Read input from the handler.
  fn read(&mut self, input: &Input) -> Result<Box<dyn Command>, InputError>;
}
