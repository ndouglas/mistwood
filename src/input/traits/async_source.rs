use crate::input::errors::input::InputError;

/// An asynchronous trait for input sources, such as standard input or a file.
///
/// Input may come from a variety of sources, such as standard input, a file,
/// or a network connection. It may also be in the form of keypresses or text
/// commands.
pub trait AsyncSource<T> {
  /// Fetch the input from the source.
  ///
  /// This method fetches input from the source asynchronously. It returns a
  /// future that resolves to the input or an error if the input cannot be
  /// fetched.
  async fn fetch_input(&mut self) -> Result<T, InputError>;
}
