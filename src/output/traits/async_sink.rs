use crate::output::error::OutputError;

/// An asynchronous trait for output sinks, such as network connections.
///
/// Output may be sent to a variety of sinks, such as standard output, a file,
/// or a network connection. It may also be in the form of text, images, or
/// other data.
pub trait AsyncSink<T> {
  /// Send the output to the source.
  ///
  /// This method sends output to the source asynchronously. It returns a
  /// future that resolves to nothing or an error if the output cannot be sent.
  async fn send_output(&mut self, output: T) -> Result<(), OutputError>;
}
