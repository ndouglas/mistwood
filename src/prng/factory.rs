use crate::di::prelude::Builder;
use crate::prng::_traits::factory::Factory as FactoryTrait;
use rand::prelude::*;
use rand::rngs::mock::StepRng;

/// A pseudorandom number generator factory.
#[derive(Debug, Clone, Copy, Default)]
pub struct Factory;

impl Factory {
  /// Creates a new pseudorandom number generator factory.
  pub fn new() -> Self {
    Self
  }
}

impl FactoryTrait for Factory {
  /// Creates a "seedable" pseudorandom number generator, which generates the same
  /// sequence of numbers given the same seed.
  fn create_seedable_rng(&self, seed: u64) -> Box<dyn RngCore + Send + Sync> {
    Box::new(StdRng::seed_from_u64(seed))
  }

  /// Creates a pseudorandom number generator that generates a sequence of numbers
  /// starting from `start` and incrementing by `step`.
  fn create_step_rng(&self, start: u64, step: u64) -> Box<dyn RngCore + Send + Sync> {
    Box::new(StepRng::new(start, step))
  }
}

/// A builder for the `Factory`.
impl Builder for Factory {
  type Input = ();
  type Output = Factory;

  fn build(_: Self::Input) -> Self::Output {
    Factory::new()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_create_seedable_rng() {
    let factory = Factory::default();
    let mut rng = factory.create_seedable_rng(42);
    assert_eq!(rng.next_u32(), 572990626);
    assert_eq!(rng.next_u32(), 2261546851);
    assert_eq!(rng.next_u32(), 1068323197);
  }

  #[test]
  fn test_create_step_rng() {
    let factory = Factory::default();
    let mut rng = factory.create_step_rng(42, 13);
    assert_eq!(rng.next_u32(), 42);
    assert_eq!(rng.next_u32(), 55);
    assert_eq!(rng.next_u32(), 68);
  }
}
