#[macro_export]
macro_rules! value_list_impl {
  ($trait_name:ident, $item_trait:ident, $serde_name:expr, $return_type:ty) => {
    #[typetag::serde(name = $serde_name)]
    impl $trait_name for Vec<Box<dyn $item_trait>> {
      fn evaluate(&self) -> Result<Vec<$return_type>, anyhow::Error> {
        let mut evaluated_values = Vec::new();
        for argument in self {
          evaluated_values.push(argument.evaluate()?);
        }
        Ok(evaluated_values)
      }
    }
  };
}

#[macro_export]
macro_rules! value_map_impl {
  ($trait_name:ident, $item_trait:ident, $serde_name:expr, $return_type:ty) => {
    #[typetag::serde(name = $serde_name)]
    impl $trait_name for std::collections::HashMap<String, Box<dyn $item_trait>> {
      fn evaluate(&self) -> Result<std::collections::HashMap<String, $return_type>, anyhow::Error> {
        let mut evaluated_values = std::collections::HashMap::new();
        for (key, argument) in self {
          evaluated_values.insert(key.clone(), argument.evaluate()?);
        }
        Ok(evaluated_values)
      }
    }
  };
}

#[macro_export]
macro_rules! value_script_impl {
  ($trait_name:ident, $item_trait:ident, $serde_name:expr, $return_type:ty, $($match_arm:tt)*) => {
    #[typetag::serde(name = $serde_name)]
    impl $trait_name for String {
      fn evaluate(&self) -> Result<$return_type, anyhow::Error> {
        use mlua::prelude::*;
        let lua = Lua::new();
        let value = lua.load(self).eval()?;
        match value {
          $($match_arm)*,
          _ => Err(anyhow!("Unsupported return type {} from lua function (expected {}).", value.type_name(), stringify!($return_type)))
        }
      }
    }
  };
}
