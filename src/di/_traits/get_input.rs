use crate::di::_traits::input_provider::InputProvider;
use crate::di::_types::Object;

/// A trait for retrieving inputs for dependency injection objects.
pub trait GetInput: Sized {
  fn get_input<P: InputProvider>(provider: &P) -> Option<Self>;
}

impl<T: 'static> GetInput for Object<T> {
  fn get_input<P: InputProvider>(provider: &P) -> Option<Self> {
    provider.provide::<T>()
  }
}

impl GetInput for () {
  fn get_input<P: InputProvider>(_provider: &P) -> Option<Self> {
    Some(())
  }
}

impl<S: GetInput, T: GetInput> GetInput for (S, T) {
  fn get_input<P: InputProvider>(provider: &P) -> Option<Self> {
    S::get_input(provider).and_then(|s| T::get_input(provider).and_then(|t| Some((s, t))))
  }
}
