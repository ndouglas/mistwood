/// Error types for Messaging.
pub mod _error;
/// Macros for Messaging.
#[macro_use]
pub mod _macros;
/// Includes Messaging-specific traits.
pub mod _traits;
/// The Gravity of the message indicates the general seriousness or severity
/// of the situation. It is somewhat akin to a loglevel.
pub mod gravity;
/// Message encapsulates a message to be sent to the user.
pub mod message;
/// This module contains a list of predefined templates.
pub mod stock_templates;
/// The Template Processor is a wrapper around Handlebars.
pub mod template_processor;
/// The Template Provider proxies requests to Template Lists.
pub mod template_provider;

/// Prelude.
pub mod prelude {
  pub use super::_error::MessagingError;
  pub use super::_macros::*;
  pub use super::_traits::template_list::TemplateList;
  pub use super::_traits::template_processor::TemplateProcessor as TemplateProcessorTrait;
  pub use super::gravity::Gravity;
  pub use super::message::Message;
  pub use super::stock_templates::thanks_for_playing::ThanksForPlaying;
  pub use super::template_processor::TemplateProcessor;
  pub use super::template_provider::TemplateProvider;
}
