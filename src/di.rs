/// A container for dependency injection.
pub mod container;
/// Errors used by the DI system.
pub mod error;
/// Traits required by the DI system.
pub mod traits;
/// Types required by the DI system.
pub mod types;

/// Prelude.
pub mod prelude {
  pub use super::container::Container;
  pub use super::error::DiError;
  pub use super::traits::builder::Builder;
  pub use super::traits::get_input::GetInput;
  pub use super::traits::input_provider::InputProvider;
  pub use super::types::Object;
}
