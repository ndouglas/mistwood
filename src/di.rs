/// Traits required by the DI system.
pub mod _traits;
/// Types required by the DI system.
pub mod _types;
/// A container for dependency injection.
pub mod container;

/// Prelude.
pub mod prelude {
  pub use super::_traits::builder::Builder;
  pub use super::_traits::get_input::GetInput;
  pub use super::_traits::input_provider::InputProvider;
  pub use super::_types::Object;
  pub use super::container::Container;
}
