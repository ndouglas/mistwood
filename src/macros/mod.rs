#[macro_export]
macro_rules! define_list_argument_trait_and_impl {
  ($trait_name:ident, $item_trait:ident, $serde_name:expr) => {
    #[typetag::serde(tag = "type")]
    pub trait $trait_name {
      fn value(&self) -> Result<&Vec<Box<dyn $item_trait>>, AnyError>;
    }

    #[typetag::serde(name = $serde_name)]
    impl $trait_name for Vec<Box<dyn $item_trait>> {
      fn value(&self) -> Result<&Vec<Box<dyn $item_trait>>, AnyError> {
        Ok(self)
      }
    }
  };
}

#[macro_export]
macro_rules! define_map_argument_trait_and_impl {
  ($trait_name:ident, $item_trait:ident, $serde_name:expr) => {
    #[typetag::serde(tag = "type")]
    pub trait $trait_name {
      fn value(&self) -> Result<&HashMap<String, Box<dyn $item_trait>>, AnyError>;
    }

    #[typetag::serde(name = $serde_name)]
    impl $trait_name for HashMap<String, Box<dyn $item_trait>> {
      fn value(&self) -> Result<&HashMap<String, Box<dyn $item_trait>>, AnyError> {
        Ok(self)
      }
    }
  };
}

#[macro_export]
macro_rules! define_argument_for_operation {
  ($trait_name:ident, $operation:ident, $return_type:expr) => {
    #[typetag::serde]
    impl $trait_name for $operation {
      fn value(&self) -> Result<$return_type, AnyError> {
        self.execute()?.value()
      }
    }
  };
}
