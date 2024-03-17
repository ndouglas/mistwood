#[macro_export]
macro_rules! operation_value_impl {
  ($trait_name:ident, $operation:ident, $return_type:expr) => {
    #[typetag::serde]
    impl $trait_name for $operation {
      fn evaluate(&self) -> Result<$return_type, AnyError> {
        self.execute()?.evaluate()
      }
    }
  };
}
