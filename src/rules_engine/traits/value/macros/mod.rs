#[macro_export]
macro_rules! value_list_trait {
  ($trait_name:ident, $return_type:ty) => {
    #[typetag::serde(tag = "type")]
    pub trait $trait_name {
      fn evaluate(&self) -> Result<Vec<$return_type>, anyhow::Error>;
    }
  };
}

#[macro_export]
macro_rules! value_map_trait {
  ($trait_name:ident, $return_type:ty) => {
    #[typetag::serde(tag = "type")]
    pub trait $trait_name {
      fn evaluate(&self) -> Result<std::collections::HashMap<String, $return_type>, anyhow::Error>;
    }
  };
}

#[macro_export]
macro_rules! value_script_trait {
  ($trait_name:ident, $return_type:ty) => {
    #[typetag::serde(tag = "type")]
    pub trait $trait_name {
      fn evaluate(&self) -> Result<$return_type, anyhow::Error>;
    }
  };
}
