use handlebars::RenderError;
use thiserror::Error as ThisError;

/// Errors for the Messaging system.
#[derive(ThisError, Debug)]
pub enum MessagingError {
  /// The template list does not actually contain any templates.
  #[error("no templates in template list")]
  NoTemplatesInTemplateList,
  /// The template failed to render due to a Handlebars RenderError.
  #[error("failed to render template: {}", .0)]
  TemplateRenderError(#[from] RenderError),
}
