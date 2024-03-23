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
///
/// struct MyBuilder;
///
/// impl Builder for MyBuilder {
///   type Input = ();
///   type Output = String;
///
///   fn build(_: Self::Input) -> Result<Self::Output, DiError> {
///     Ok("Hello, world!".to_string())
///   }
/// }
/// ```
pub trait Builder {
  /// The input type for the builder. This will normally be a single type or a
  /// tuple of types. This is used to specify the dependencies of the service
  /// being built.
  type Input: GetInput;
  /// The output type for the builder. This is the type of the service being
  /// built.
  type Output: 'static;
  /// Build the service.
  fn build(input: Self::Input) -> Result<Self::Output, DiError>;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_builder() {
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
}
