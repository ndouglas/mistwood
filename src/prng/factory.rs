use crate::di::prelude::Builder;
use rand::prelude::*;
use rand::rngs::mock::StepRng;
use std::any::type_name;
use std::hash::{DefaultHasher, Hash, Hasher};

/// A pseudorandom number generator factory.
#[derive(Debug, Clone, Copy, Default)]
pub struct Factory;

impl Factory {
  /// Creates a new pseudorandom number generator factory.
  pub fn new() -> Self {
    Self
  }

  /// Creates a seedable pseudorandom number generator based on the hash of the type name.
  pub fn create_seedable_rng_from_type<T: 'static>(&self, seed: u64) -> Box<dyn RngCore> {
    let type_name = type_name::<T>();
    let mut hasher = DefaultHasher::new();
    type_name.hash(&mut hasher);
    let hash = hasher.finish();
    let seed = seed.wrapping_add(hash);
    Box::new(StdRng::seed_from_u64(seed))
  }

  /// Creates a "seedable" pseudorandom number generator, which generates the same
  /// sequence of numbers given the same seed.
  pub fn create_seedable_rng(&self, seed: u64) -> Box<dyn RngCore> {
    Box::new(StdRng::seed_from_u64(seed))
  }

  /// Creates a pseudorandom number generator that generates a sequence of numbers
  /// starting from `start` and incrementing by `step`.
  pub fn create_step_rng(&self, start: u64, step: u64) -> Box<dyn RngCore> {
    Box::new(StepRng::new(start, step))
  }
}

/// A builder for the `TemplateProcessor`.
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

  #[test]
  fn test_create_seedable_rng_from_type() {
    let factory = Factory::default();
    let mut rng = factory.create_seedable_rng_from_type::<u32>(42);
    assert_eq!(rng.next_u32(), 2705398361);
    assert_eq!(rng.next_u32(), 3827724650);
    assert_eq!(rng.next_u32(), 2386396018);
  }
}
