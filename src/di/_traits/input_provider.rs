use crate::di::_types::Object;

/// Trait for providing input in the dependency injection system.
pub trait InputProvider {
  fn provide<T: 'static>(&self) -> Option<Object<T>>;
}
