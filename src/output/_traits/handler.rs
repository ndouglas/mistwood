use crate::output::_error::OutputError;
use crate::output::output::Output;

/// A trait for output handlers.
///
/// This trait will be responsible for taking Output instances and processing
/// them into a final form to be displayed to the user. It can handle format-
/// ting, templating, and deciding on the output medium.
pub trait Handler {
  /// Processes the output.
  fn handle(&mut self, output: &Output) -> Result<(), OutputError>;
}
