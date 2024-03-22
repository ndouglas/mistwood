use thiserror::Error as ThisError;

/// Errors for the Messaging system.
#[derive(ThisError, Clone, Copy, Debug)]
pub enum MessagingError {
  /// The template provider does not have any templates in its TEMPLATES slice.
  #[error("no templates in template provider")]
  NoTemplatesInTemplateProvider,
}
