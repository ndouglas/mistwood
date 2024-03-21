use crate::di::_traits::builder::Builder;
use crate::di::_traits::get_input::GetInput;
use crate::di::_traits::input_provider::InputProvider;
use crate::di::_types::Object;
use crate::prelude::TypeMap;
use std::sync::{Arc, Mutex};

/// A container for dependency injection.
///
/// This is a simple wrapper around `TypeMap` that allows for type-safe
/// dependency injection by storing and retrieving values by type.
///
/// @see <https://willcrichton.net/rust-api-type-patterns/registries.html>
///
/// # Example
///
/// ```rust
/// use mistwood::prelude::Builder;
/// use mistwood::prelude::Container;
///
/// #[derive(Debug, PartialEq)]
/// struct NewString(String);
///
/// impl Builder for NewString {
///   type Input = ();
///   type Output = NewString;
///   fn build(_: Self::Input) -> Self::Output {
///     NewString("Hello, world!".to_string())
///   }
/// }
///
/// let mut container = Container::new();
/// container.build::<NewString>();
/// let string = container.get::<NewString>().unwrap();
/// assert_eq!(string.lock().unwrap().0, "Hello, world!".to_string());
/// ```
#[derive(Default)]
pub struct Container(pub TypeMap);

impl Container {
  /// Create a new, empty `Container`.
  pub fn new() -> Self {
    Self(TypeMap::new())
  }

  /// Set a value of type `T`.
  pub fn build<T: Builder>(&mut self) -> Option<Object<T::Output>> {
    let input = T::Input::get_input(self)?;
    let object = T::build(input);
    let sync_object = Arc::new(Mutex::new(object));
    self.0.set::<Object<T::Output>>(sync_object.clone());
    Some(sync_object)
  }

  /// Get a reference to a value of type `T`.
  pub fn get<T: 'static>(&self) -> Option<Object<T>> {
    self.0.get::<Object<T>>().cloned()
  }
}

impl InputProvider for Container {
  /// Provide the requested input from the container.
  fn provide<T: 'static>(&self) -> Option<Object<T>> {
    self.0.get::<Object<T>>().cloned()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[derive(Debug, PartialEq)]
  struct NewString(String);

  impl Builder for NewString {
    type Input = ();
    type Output = NewString;
    fn build(_: Self::Input) -> Self::Output {
      NewString("Hello, world!".to_string())
    }
  }

  #[derive(Debug)]
  struct NewInt(i32);

  impl Builder for NewInt {
    type Input = ();
    type Output = NewInt;
    fn build(_: Self::Input) -> Self::Output {
      NewInt(42)
    }
  }

  #[derive(Debug)]
  struct FactProvider {
    new_string: Object<NewString>,
    new_int: Object<NewInt>,
  }

  impl Builder for FactProvider {
    type Input = (Object<NewString>, Object<NewInt>);
    type Output = FactProvider;
    fn build((new_string, new_int): Self::Input) -> Self::Output {
      FactProvider { new_string, new_int }
    }
  }

  #[test]
  fn test_container() {
    let mut container = Container::new();
    container.build::<NewString>();
    let string = container.get::<NewString>().unwrap();
    assert_eq!(string.lock().unwrap().0, "Hello, world!".to_string());
  }

  #[test]
  fn test_fact_provider() {
    let mut container = Container::new();
    container.build::<NewString>();
    container.build::<NewInt>();
    container.build::<FactProvider>();
    let fact_provider = container.get::<FactProvider>().unwrap();
    assert_eq!(
      fact_provider.lock().unwrap().new_string.lock().unwrap().0,
      "Hello, world!".to_string()
    );
    assert_eq!(fact_provider.lock().unwrap().new_int.lock().unwrap().0, 42);
  }
}
