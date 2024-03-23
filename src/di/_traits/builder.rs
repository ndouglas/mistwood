use crate::di::_error::DiError;
use crate::di::_traits::get_input::GetInput;

/// A trait for a builder; this represents a constructor within our DI system.
///
/// @see <https://willcrichton.net/rust-api-type-patterns/registries.html>
///
/// # Example
///
/// ```rust
/// use mistwood::di::prelude::Builder;
/// use mistwood::di::prelude::DiError;
/// use mistwood::di::prelude::Object;
/// use std::string::String;
///
/// struct MyService(Object<Box<dyn MyString>>);
///
/// struct StringWrapper(String);
/// trait MyString {}
///
/// impl MyString for StringWrapper {}
///
/// impl Builder for StringWrapper {
///   type Input = ();
///   type Output = Box<dyn MyString>;
///   fn build(_: Self::Input) -> Result<Self::Output, DiError> {
///     Ok(Box::new(StringWrapper("Hello, world!".to_string())))
///   }
/// }
///
/// impl Builder for MyService {
///   type Input = (Object<Box<dyn MyString>>,);
///   type Output = MyService;
///
///   fn build((my_string,): Self::Input) -> Result<Self::Output, DiError> {
///     Ok(MyService(my_string))
///   }
/// }
/// ```
pub trait Builder {
  /// The input type for the builder. This will normally be a single type or a
  /// tuple of types, each of which is likely to be Object<Box<T>>. This is
  /// used to specify the dependencies of the service being built.
  ///
  /// For instance:
  /// - `type Input = ();`
  /// - `type Input = (Object<Box<dyn MyString>>,);`
  /// - `type Input = (Object<Box<dyn Database>>, Object<Box<dyn CacheService>>);`
  type Input: GetInput;
  /// The output type for the builder. This is the type of the service being
  /// built. This is often a boxed trait object, but it could be any type.
  ///
  /// For instance:
  /// - `type Output = WebServer;`
  /// - `type Output = Box<dyn Database>;`
  type Output: 'static;
  /// Build the service.
  fn build(input: Self::Input) -> Result<Self::Output, DiError>;
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::di::prelude::Object;
  use crate::test::init as test_init;
  use std::sync::{Arc, Mutex};

  #[test]
  fn test_builder() {
    test_init();
    struct MyBuilder;
    impl Builder for MyBuilder {
      type Input = ();
      type Output = String;
      fn build(_: Self::Input) -> Result<Self::Output, DiError> {
        Ok("Hello, world!".to_string())
      }
    }
    assert_eq!(MyBuilder::build(()).unwrap(), "Hello, world!".to_string());
  }

  #[test]
  fn test_builder_with_input() {
    test_init();
    struct MyStruct(Object<Box<dyn MyString>>);
    pub trait MyString {}
    impl MyString for String {}
    impl Builder for String {
      type Input = ();
      type Output = Box<dyn MyString>;
      fn build(_: Self::Input) -> Result<Self::Output, DiError> {
        Ok(Box::new("Hello, world!".to_string()))
      }
    }
    impl Builder for MyStruct {
      type Input = (Object<Box<dyn MyString>>,);
      type Output = MyStruct;
      fn build((my_string,): Self::Input) -> Result<Self::Output, DiError> {
        Ok(MyStruct(my_string))
      }
    }
    let build_string = Arc::new(Mutex::new(String::build(()).unwrap()));
    let built_struct = MyStruct::build((build_string,)).unwrap();
    assert!(built_struct.0.lock().is_ok());
  }
}
