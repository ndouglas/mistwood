use thiserror::Error as ThisError;

/// Errors for the Messaging system.
#[derive(ThisError, Clone, Copy, Debug)]
pub enum MessagingError {
  /// The template list does not actually contain any templates.
  #[error("no templates in template list")]
  NoTemplatesInTemplateList,
}
