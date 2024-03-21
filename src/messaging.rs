/// Includes Messaging-specific traits.
pub mod _traits;
/// The Gravity of the message indicates the general seriousness or severity
/// of the situation. It is somewhat akin to a loglevel.
pub mod gravity;
/// The Template contains the specific format of the message.
pub mod template;
/// The Template Provider Registry is a registry of Template Providers.
pub mod template_provider_registry;

/// Prelude.
pub mod prelude {
  pub use super::_traits::template_provider::TemplateProvider;
  pub use super::gravity::Gravity;
  pub use super::template::Template;
  pub use super::template_provider_registry::TemplateProviderRegistry;
}
