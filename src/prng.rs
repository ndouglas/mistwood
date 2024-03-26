/// A pseudorandom number generator factory.
pub mod factory;
/// A registry of pseudorandom number generators.
pub mod registry;
/// Traits.
pub mod traits;
/// Types.
pub mod types;

/// Prelude.
pub mod prelude {
  pub use super::factory::Factory;
  pub use super::registry::Registry;
  pub use super::types::SafePrng;
}
