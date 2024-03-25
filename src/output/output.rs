use crate::messaging::prelude::Message;

/// An enum representing the output.
#[derive(Debug)]
pub enum Output {
  /// A message to be output.
  Message(Message),
}
