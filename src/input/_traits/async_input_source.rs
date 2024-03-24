use crate::input::_errors::input::InputError;

/// An asynchronous trait for input sources, such as standard input or a file.
///
/// Input may come from a variety of sources, such as standard input, a file,
/// or a network connection. It may also be in the form of keypresses or text
/// commands.
pub trait AsyncInputSource {
  /// Fetch the input from the source.
  async fn fetch_input(&self) -> Result<String, InputError>;
}
