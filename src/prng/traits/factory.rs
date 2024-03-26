use rand::prelude::*;

/// A pseudorandom number generator factory.
pub trait Factory {
  /// Creates a "seedable" pseudorandom number generator, which generates the same
  /// sequence of numbers given the same seed.
  fn create_seedable_rng(&self, seed: u64) -> Box<dyn RngCore + Send + Sync>;

  /// Creates a pseudorandom number generator that generates a sequence of numbers
  /// starting from `start` and incrementing by `step`.
  fn create_step_rng(&self, start: u64, step: u64) -> Box<dyn RngCore + Send + Sync>;
}
