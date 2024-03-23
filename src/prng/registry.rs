use crate::prng::_types::SafePrng;
use crate::prng::factory::Factory;
use rand::RngCore;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::sync::{Arc, Mutex};

/// A registry of pseudorandom number generators.
///
/// This is a simple wrapper around `HashMap` that allows us to store and
/// retrieve pseudorandom number generators by a string key.
#[derive(Default)]
pub struct Registry {
  prngs: HashMap<String, SafePrng>,
  /// A factory for creating pseudorandom number generators.
  pub factory: Factory,
}

impl Registry {
  /// Create a new, empty `Registry`.
  pub fn new(factory: Factory) -> Self {
    let prngs = HashMap::new();
    Self { prngs, factory }
  }

  /// Set a prng for a key.
  pub fn set(&mut self, key: &str, rng: Box<dyn RngCore + Send + Sync>) {
    let prng = Arc::new(Mutex::new(rng));
    self.prngs.insert(key.to_string(), prng);
  }

  /// Check if a prng exists for a key.
  pub fn has(&self, key: &str) -> bool {
    self.prngs.contains_key(key)
  }

  /// Get a mutable reference to a prng for a type `T`.
  pub fn get_mut(&mut self, key: &str) -> Option<SafePrng> {
    self.prngs.get_mut(key).cloned()
  }

  /// Register a seedable prng for a key.
  pub fn register_seedable_rng(&mut self, key: &str, seed: u64) {
    let rng = self.factory.create_seedable_rng(seed);
    self.set(key, rng);
  }

  /// Register a step prng for a key.
  pub fn register_step_rng(&mut self, key: &str, start: u64, step: u64) {
    let rng = self.factory.create_step_rng(start, step);
    self.set(key, rng);
  }

  /// Register a seedable prng for a key based on the type `T`.
  pub fn register_seedable_rng_from_type<T: 'static>(&mut self, key: &str, seed: u64) {
    let rng = self.factory.create_seedable_rng_from_type::<T>(seed);
    self.set(key, rng);
  }
}

impl Debug for Registry {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "Registry {{ ... }}")
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::test::init as test_init;
  use pretty_assertions::assert_eq;
  use rand::rngs::mock::StepRng;

  #[test]
  fn test_registry() {
    test_init();
    let mut registry = Registry::default();
    let rng = StepRng::new(42, 13);
    let prng = Box::new(rng);
    registry.set("u32", prng);
    assert!(registry.has("u32"));
    let prng = registry.get_mut("u32").unwrap();
    assert_eq!(prng.lock().unwrap().next_u32(), 42);
  }
}
