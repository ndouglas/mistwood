use crate::input::_errors::input::InputError;

/// A trait for input sources, such as standard input or a file.
///
/// Input may come from a variety of sources, such as standard input, a file,
/// or a network connection. It may also be in the form of keypresses or text
/// commands.
pub trait InputSource {
  /// Fetch the input from the source.
  ///
  /// This method fetches input from the source, returning it as a String; it
  /// blocks until input is available or returns an error if the input cannot
  /// be fetched.
  ///
  /// The implementation of this method will vary depending on the input source
  /// (e.g., reading from stdin, a network socket, or a test script).
  fn fetch_input(&mut self) -> Result<String, InputError>;
}
