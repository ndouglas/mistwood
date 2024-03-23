/// Types for pseudorandom number generators.
pub mod _types;
/// A pseudorandom number generator factory.
pub mod factory;
/// A registry of pseudorandom number generators.
pub mod registry;

/// Prelude.
pub mod prelude {
  pub use super::_types::SafePrng;
  pub use super::factory::Factory;
  pub use super::registry::Registry;
}
