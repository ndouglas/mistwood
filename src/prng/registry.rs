use crate::prng::_types::SafePrng;
use rand::RngCore;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::sync::{Arc, Mutex};

/// A registry of pseudorandom number generators.
///
/// This is a simple wrapper around `TypeMap` that allows for type-safe
/// dependency injection by storing and retrieving prngs by the type
/// that is calling for them.
///
/// This allows for the creation of multiple prngs with different seeds
/// and types, and for the retrieval of the same prng by the same type
/// that created it.
#[derive(Default)]
pub struct Registry(HashMap<TypeId, SafePrng>);

impl Registry {
  /// Create a new, empty `Registry`.
  pub fn new() -> Self {
    Self(HashMap::new())
  }

  /// Set a prng for a type `T`.
  pub fn set<T: Any + 'static>(&mut self, rng: Box<dyn RngCore + Send + Sync>) {
    let prng = Arc::new(Mutex::new(rng));
    self.0.insert(TypeId::of::<T>(), prng);
  }

  /// Check if a prng for a type `T` is present.
  pub fn has<T: Any + 'static>(&self) -> bool {
    self.0.contains_key(&TypeId::of::<T>())
  }

  /// Get a mutable reference to a prng for a type `T`.
  pub fn get_mut<T: Any + 'static>(&mut self) -> Option<SafePrng> {
    self.0.get_mut(&TypeId::of::<T>()).cloned()
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
    registry.set::<u32>(prng);
    assert!(registry.has::<u32>());
    let prng = registry.get_mut::<u32>().unwrap();
    assert_eq!(prng.lock().unwrap().next_u32(), 42);
  }
}
