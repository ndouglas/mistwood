use crate::di::_traits::input_provider::InputProvider;
use crate::di::_types::Object;

/// A trait for retrieving inputs for dependency injection objects.
pub trait GetInput: Sized {
  /// Get the input for a type `T` from the given `InputProvider`.
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

macro_rules! impl_get_input_for_tuple {
  ( $($name:ident),* ) => {
    /// Implement `GetInput` for a tuple of types.
    impl<$($name: GetInput),*> GetInput for ($($name,)*) {
      /// Get the input for a type `T` from the given `InputProvider`.
      fn get_input<IP: InputProvider>(provider: &IP) -> Option<Self> {
        Some(($($name::get_input(provider)?,)*))
      }
    }
  };
}

impl_get_input_for_tuple!(A);
impl_get_input_for_tuple!(A, B);
impl_get_input_for_tuple!(A, B, C);
impl_get_input_for_tuple!(A, B, C, D);
impl_get_input_for_tuple!(A, B, C, D, E);
impl_get_input_for_tuple!(A, B, C, D, E, F);
impl_get_input_for_tuple!(A, B, C, D, E, F, G);
impl_get_input_for_tuple!(A, B, C, D, E, F, G, H);
impl_get_input_for_tuple!(A, B, C, D, E, F, G, H, I);
impl_get_input_for_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_get_input_for_tuple!(A, B, C, D, E, F, G, H, I, J, K);
impl_get_input_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);
impl_get_input_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M);
impl_get_input_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
impl_get_input_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
impl_get_input_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
impl_get_input_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q);
impl_get_input_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R);
impl_get_input_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S);
impl_get_input_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T);
impl_get_input_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U);
impl_get_input_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V);
impl_get_input_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W);
impl_get_input_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X);
impl_get_input_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y);
impl_get_input_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);
