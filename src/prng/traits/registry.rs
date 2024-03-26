use crate::prng::types::BoxedPrng;
use crate::prng::types::SafePrng;

/// A registry of pseudorandom number generators.
pub trait Registry {
  /// Set a prng for a key.
  fn set(&mut self, key: &str, rng: BoxedPrng);

  /// Check if a prng exists for a key.
  fn has(&self, key: &str) -> bool;

  /// Get a mutable reference to a prng for a type `T`.
  fn get_mut(&mut self, key: &str) -> Option<SafePrng>;

  /// Register a seedable prng for a key.
  fn register_seedable_rng(&mut self, key: &str, seed: u64);

  /// Register a step prng for a key.
  fn register_step_rng(&mut self, key: &str, start: u64, step: u64);
}
