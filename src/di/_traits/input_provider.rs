use crate::di::_error::DiError;
use crate::di::_types::Object;

/// Trait for providing inputs (dependencies) to a builder.
pub trait InputProvider {
  /// Provide an input for a type `T`.
  fn provide<T: 'static>(&self) -> Result<Object<T>, DiError>;
}
