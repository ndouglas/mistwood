use crate::di::prelude::Builder;
use crate::di::prelude::DiError;
use crate::di::prelude::Object;
use crate::prng::traits::factory::Factory as FactoryTrait;
use crate::prng::traits::registry::Registry as RegistryTrait;
use crate::prng::types::BoxedPrng;
use crate::prng::types::SafePrng;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::sync::{Arc, Mutex};

/// A registry of pseudorandom number generators.
///
/// This is a simple wrapper around `HashMap` that allows us to store and
/// retrieve pseudorandom number generators by a string key.
pub struct Registry {
  prngs: HashMap<String, SafePrng>,
  /// A factory for creating pseudorandom number generators.
  pub factory: Object<Box<dyn FactoryTrait>>,
}

impl Registry {
  /// Create a new, empty `Registry`.
  pub fn new(factory: &Object<Box<dyn FactoryTrait>>) -> Self {
    let prngs = HashMap::new();
    let factory = factory.clone();
    Self { prngs, factory }
  }
}

impl RegistryTrait for Registry {
  /// Set a prng for a key.
  fn set(&mut self, key: &str, rng: BoxedPrng) {
    let prng = Arc::new(Mutex::new(rng));
    self.prngs.insert(key.to_string(), prng);
  }

  /// Check if a prng exists for a key.
  fn has(&self, key: &str) -> bool {
    self.prngs.contains_key(key)
  }

  /// Get a mutable reference to a prng for a type `T`.
  fn get_mut(&mut self, key: &str) -> Option<SafePrng> {
    self.prngs.get_mut(key).cloned()
  }

  /// Register a seedable prng for a key.
  fn register_seedable_rng(&mut self, key: &str, seed: u64) {
    let rng = self.factory.lock().unwrap().create_seedable_rng(seed);
    self.set(key, rng);
  }

  /// Register a step prng for a key.
  fn register_step_rng(&mut self, key: &str, start: u64, step: u64) {
    let rng = self.factory.lock().unwrap().create_step_rng(start, step);
    self.set(key, rng);
  }
}

/// A builder for the `Registry`.
impl Builder for Registry {
  type Input = (Object<Box<dyn FactoryTrait>>,);
  type Output = Box<dyn RegistryTrait>;

  fn build((factory,): Self::Input) -> Result<Self::Output, DiError> {
    Ok(Box::new(Registry::new(&factory)))
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
  use crate::di::container::Container;
  use crate::prng::factory::Factory;
  use crate::test::init as test_init;
  use pretty_assertions::assert_eq;
  use rand::rngs::mock::StepRng;

  #[test]
  fn test_registry() {
    test_init();
    let factory: Box<dyn FactoryTrait> = Box::new(Factory::default());
    let object_factory = Arc::new(Mutex::new(factory));
    let mut registry = Registry::new(&object_factory);
    let rng = StepRng::new(42, 13);
    let prng = Box::new(rng);
    registry.set("u32", prng);
    assert!(registry.has("u32"));
    let prng = registry.get_mut("u32").unwrap();
    assert_eq!(prng.lock().unwrap().next_u32(), 42);
  }

  #[test]
  fn test_register_seedable_rng() {
    test_init();
    let factory: Box<dyn FactoryTrait> = Box::new(Factory::default());
    let object_factory = Arc::new(Mutex::new(factory));
    let mut registry = Registry::new(&object_factory);
    registry.register_seedable_rng("u32", 42);
    assert!(registry.has("u32"));
    let prng = registry.get_mut("u32").unwrap();
    assert_eq!(prng.lock().unwrap().next_u32(), 572990626);
  }

  #[test]
  fn test_register_step_rng() {
    test_init();
    let factory: Box<dyn FactoryTrait> = Box::new(Factory::default());
    let object_factory = Arc::new(Mutex::new(factory));
    let mut registry = Registry::new(&object_factory);
    registry.register_step_rng("u32", 42, 13);
    assert!(registry.has("u32"));
    let prng = registry.get_mut("u32").unwrap();
    assert_eq!(prng.lock().unwrap().next_u32(), 42);
  }

  #[test]
  fn test_registry_debug() {
    test_init();
    let factory: Box<dyn FactoryTrait> = Box::new(Factory::default());
    let object_factory = Arc::new(Mutex::new(factory));
    let registry = Registry::new(&object_factory);
    assert_eq!(format!("{:?}", registry), "Registry { ... }");
  }

  #[test]
  fn test_registry_builder() {
    test_init();
    let mut container = Container::new();
    let factory: Object<Box<dyn FactoryTrait>> = container.build::<Factory>().unwrap();
    let registry: Object<Box<dyn RegistryTrait>> = container.build::<Registry>().unwrap();
    let registry2 = container.get::<Registry>().unwrap();
    assert_eq!(factory.lock().unwrap().create_seedable_rng(42).next_u32(), 572990626);
    registry.lock().unwrap().register_seedable_rng("foo", 42);
    let foo = registry.lock().unwrap().get_mut("foo").unwrap();
    registry2.lock().unwrap().register_seedable_rng("bar", 42);
    let bar = registry2.lock().unwrap().get_mut("bar").unwrap();
    assert_eq!(foo.lock().unwrap().next_u32(), 572990626);
    assert_eq!(bar.lock().unwrap().next_u32(), 572990626);
  }
}
