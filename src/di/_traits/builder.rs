use crate::di::_traits::get_input::GetInput;

/// A trait for a builder; this represents a constructor within our DI system.
///
/// @see <https://willcrichton.net/rust-api-type-patterns/registries.html>
///
/// # Example
///
/// ```rust
/// use mistwood::prelude::Builder;
///
/// struct MyBuilder;
///
/// impl Builder for MyBuilder {
///   type Input = ();
///   type Output = String;
///
///   fn build(_: Self::Input) -> Self::Output {
///     "Hello, world!".to_string()
///   }
/// }
/// ```
pub trait Builder {
  type Input: GetInput;
  type Output: 'static;
  fn build(input: Self::Input) -> Self::Output;
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

      fn build(_: Self::Input) -> Self::Output {
        "Hello, world!".to_string()
      }
    }

    assert_eq!(MyBuilder::build(()), "Hello, world!".to_string());
  }
}
