use crate::input::errors::input::InputError;

/// A trait for synchronous input sources, such as standard input or a file.
///
/// Input may come from a variety of sources, such as standard input, a file,
/// or a network connection. It may also be in the form of keypresses or text
/// commands or something else.
pub trait SyncSource<T> {
  /// Fetch the input from the source.
  ///
  /// This method fetches input from the source; it blocks until input is ready
  /// or returns an error if the input cannot be fetched.
  ///
  /// The implementation of this method will vary depending on the input source
  /// (e.g., reading from stdin, a network socket, or a test script) and on the
  /// type of input (e.g., text, binary data, or keypresses).
  fn fetch_input(&mut self) -> Result<T, InputError>;
}
