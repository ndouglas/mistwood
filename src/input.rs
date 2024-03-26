/// Errors for input processing.
pub mod errors;
/// An enum representing the input.
#[allow(clippy::module_inception)]
pub mod input;
/// Implementations of InputSource for various input sources.
pub mod sources;
/// Traits for input processing.
pub mod traits;

/// The prelude for the input module.
pub mod prelude {
  pub use super::errors::input::InputError;
  pub use super::errors::parse::ParseError;
  pub use super::input::Input;
  pub use super::sources::generic::FileSource;
  pub use super::sources::generic::GenericSource;
  pub use super::sources::generic::StdinSource;
  pub use super::sources::string::StringSource;
  pub use super::traits::async_source::AsyncSource;
  pub use super::traits::handler::Handler as InputHandler;
  pub use super::traits::sync_source::SyncSource;
}
