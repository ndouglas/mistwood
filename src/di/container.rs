use crate::di::_error::DiError;
use crate::di::_traits::builder::Builder;
use crate::di::_traits::get_input::GetInput;
use crate::di::_traits::input_provider::InputProvider;
use crate::di::_types::Object;
use crate::prelude::TypeMap;
use std::any::type_name;
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
/// use mistwood::di::prelude::Builder;
/// use mistwood::di::prelude::Container;
/// use mistwood::di::prelude::DiError;
///
/// #[derive(Debug, PartialEq)]
/// struct NewString(String);
///
/// impl Builder for NewString {
///   type Input = ();
///   type Output = NewString;
///   fn build(_: Self::Input) -> Result<Self::Output, DiError> {
///     Ok(NewString("Hello, world!".to_string()))
///   }
/// }
///
/// let mut container = Container::new();
/// container.build::<NewString>();
/// let string = container.get::<NewString>().unwrap();
/// assert_eq!(string.lock().unwrap().0, "Hello, world!".to_string());
/// ```
#[derive(Debug, Default)]
pub struct Container(TypeMap);

impl Container {
  /// Create a new, empty `Container`.
  pub fn new() -> Self {
    Self(TypeMap::new())
  }

  /// Set a value of type `T`.
  pub fn build<T: Builder>(&mut self) -> Result<Object<T::Output>, DiError> {
    let input = T::Input::get_input(self)?;
    let object = T::build(input)?;
    let sync_object = Arc::new(Mutex::new(object));
    self.0.set::<Object<T::Output>>(sync_object.clone());
    Ok(sync_object)
  }

  /// Get a reference to a value of type `T`.
  pub fn get<T: 'static>(&self) -> Result<Object<T>, DiError> {
    self
      .0
      .get::<Object<T>>()
      .cloned()
      .ok_or_else(|| DiError::NotFound(format!("{} not found", type_name::<T>())))
  }
}

impl InputProvider for Container {
  /// Provide the requested input from the container.
  fn provide<T: 'static>(&self) -> Result<Object<T>, DiError> {
    self.get::<T>()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::di::_error::DiError;

  #[derive(Debug, PartialEq)]
  struct NewString(String);

  impl Builder for NewString {
    type Input = ();
    type Output = NewString;
    fn build(_: Self::Input) -> Result<Self::Output, DiError> {
      Ok(NewString("Hello, world!".to_string()))
    }
  }

  #[derive(Debug)]
  struct NewInt(i32);

  impl Builder for NewInt {
    type Input = ();
    type Output = NewInt;
    fn build(_: Self::Input) -> Result<Self::Output, DiError> {
      Ok(NewInt(42))
    }
  }

  #[derive(Debug)]
  struct IntContainer {
    new_int: Object<NewInt>,
  }

  impl Builder for IntContainer {
    type Input = Object<NewInt>;
    type Output = IntContainer;
    fn build(new_int: Self::Input) -> Result<Self::Output, DiError> {
      Ok(IntContainer { new_int })
    }
  }

  #[derive(Debug)]
  struct FactProvider {
    new_string: Object<NewString>,
    int_container: Object<IntContainer>,
  }

  impl Builder for FactProvider {
    type Input = (Object<NewString>, Object<IntContainer>);
    type Output = FactProvider;
    fn build((new_string, int_container): Self::Input) -> Result<Self::Output, DiError> {
      Ok(FactProvider {
        new_string,
        int_container,
      })
    }
  }

  #[test]
  fn test_container() {
    let mut container = Container::new();
    container.build::<NewString>().unwrap();
    let string = container.get::<NewString>().unwrap();
    assert_eq!(string.lock().unwrap().0, "Hello, world!".to_string());
  }

  #[test]
  fn test_fact_provider() {
    let mut container = Container::new();
    container.build::<NewString>().unwrap();
    container.build::<NewInt>().unwrap();
    container.build::<IntContainer>().unwrap();
    container.build::<FactProvider>().unwrap();
    let fact_provider = container.get::<FactProvider>().unwrap();
    assert_eq!(
      fact_provider.lock().unwrap().new_string.lock().unwrap().0,
      "Hello, world!".to_string()
    );
    assert_eq!(
      fact_provider
        .lock()
        .unwrap()
        .int_container
        .lock()
        .unwrap()
        .new_int
        .lock()
        .unwrap()
        .0,
      42
    );
  }
}
