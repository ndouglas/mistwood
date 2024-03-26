use crate::messaging::prelude::Message;
use crate::messaging::prelude::MessagingError;

/// This trait describes an object that can process a message template.
pub trait TemplateProcessor {
  /// Process a message.
  fn process_message(&self, message: &Message) -> Result<String, MessagingError>;
}
