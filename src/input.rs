/// Errors for input processing.
pub mod _errors;
/// Traits for input processing.
pub mod _traits;

/// The prelude for the input module.
pub mod prelude {
  pub use super::_errors::input::InputError;
  pub use super::_errors::parse::ParseError;
  pub use super::_traits::input_source::InputSource;
}
