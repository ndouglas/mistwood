/// Error types for Messaging.
pub mod _error;
/// Includes Messaging-specific traits.
pub mod _traits;
/// The Gravity of the message indicates the general seriousness or severity
/// of the situation. It is somewhat akin to a loglevel.
pub mod gravity;
/// Message encapsulates a message to be sent to the user.
pub mod message;
/// The Template Processor is a wrapper around Handlebars.
pub mod template_processor;
/// The Template Provider Registry is a registry of Template Providers.
pub mod template_provider_registry;

/// Prelude.
pub mod prelude {
  pub use super::_error::MessagingError;
  pub use super::_traits::template_provider::TemplateProvider;
  pub use super::gravity::Gravity;
  pub use super::message::Message;
  pub use super::template_processor::TemplateProcessor;
  pub use super::template_provider_registry::TemplateProviderRegistry;
}
