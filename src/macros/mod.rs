#[macro_export]
macro_rules! define_list_argument_trait_and_impl {
  ($trait_name:ident, $item_trait:ident, $serde_name:expr, $return_type:ty) => {
    #[typetag::serde(tag = "type")]
    pub trait $trait_name {
      fn evaluate(&self) -> Result<Vec<$return_type>, AnyError>;
    }

    #[typetag::serde(name = $serde_name)]
    impl $trait_name for Vec<Box<dyn $item_trait>> {
      fn evaluate(&self) -> Result<Vec<$return_type>, AnyError> {
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
macro_rules! define_map_argument_trait_and_impl {
  ($trait_name:ident, $item_trait:ident, $serde_name:expr, $return_type:ty) => {
    #[typetag::serde(tag = "type")]
    pub trait $trait_name {
      fn evaluate(&self) -> Result<HashMap<String, $return_type>, AnyError>;
    }

    #[typetag::serde(name = $serde_name)]
    impl $trait_name for HashMap<String, Box<dyn $item_trait>> {
      fn evaluate(&self) -> Result<HashMap<String, $return_type>, AnyError> {
        let mut evaluated_values = HashMap::new();
        for (key, argument) in self {
          evaluated_values.insert(key.clone(), argument.evaluate()?);
        }
        Ok(evaluated_values)
      }
    }
  };
}

#[macro_export]
macro_rules! define_script_argument_trait_and_string_impl {
  ($trait_name:ident, $item_trait:ident, $serde_name:expr, $return_type:ty, $($match_arm:tt)*) => {
    #[typetag::serde(tag = "type")]
    pub trait $trait_name {
      fn evaluate(&self) -> Result<$return_type, AnyError>;
    }

    #[typetag::serde(name = $serde_name)]
    impl $trait_name for String {
      fn evaluate(&self) -> Result<$return_type, AnyError> {
        use mlua::prelude::*;
        use mlua::Function;
        let lua = Lua::new();
        let function: Function = lua.load(self).eval()?;
        let value: mlua::Value = function.call(())?;
        match value {
          $($match_arm)*,
          _ => Err(anyhow!("Unsupported return type {} from lua function (expected {}).", value.type_name(), stringify!($return_type)))
        }
      }
    }
  };
}

#[macro_export]
macro_rules! define_argument_for_operation {
  ($trait_name:ident, $operation:ident, $return_type:expr) => {
    #[typetag::serde]
    impl $trait_name for $operation {
      fn evaluate(&self) -> Result<$return_type, AnyError> {
        self.execute()?.evaluate()
      }
    }
  };
}
