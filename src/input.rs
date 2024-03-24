/// Errors for input processing.
pub mod _errors;
/// Traits for input processing.
pub mod _traits;
/// Implementations of InputSource for various input sources.
pub mod sources;

/// The prelude for the input module.
pub mod prelude {
  pub use super::_errors::input::InputError;
  pub use super::_errors::parse::ParseError;
  pub use super::_traits::input_source::InputSource;
  pub use super::sources::generic::FileSource;
  pub use super::sources::generic::GenericSource;
  pub use super::sources::generic::StdinSource;
  pub use super::sources::string::StringSource;
}
