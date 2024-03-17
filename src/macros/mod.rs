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
macro_rules! define_script_argument_impl {
  ($struct_name:ident, $item_trait:ident, $serde_name:expr, $return_type:expr, $value_type:ident) => {
    #[derive(Derivative, Deserialize, Serialize)]
    pub struct $struct_name {
      script: String,
    }

    #[typetag::serde(name = $serde_name)]
    impl $item_trait for $struct_name {
      fn value(&self) -> Result<$return_type, AnyError> {
        use mlua::{Function, Lua, Value};
        let lua = Lua::new();
        let _globals = lua.globals();
        let function: Function = lua.load(&self.script).eval()?;
        let result = function.call(())?;
        // Return the result of the function call iff it matches $return_type.
        match result {
          Value::$value_type(value) => Ok(value),
          _ => Err(AnyError::msg(format!(
            "Expected {} but got {:?}",
            stringify!($return_type),
            result
          ))),
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
      fn value(&self) -> Result<$return_type, AnyError> {
        self.execute()?.value()
      }
    }
  };
}

// pub enum Value<'lua> {
//   Nil,
//   Boolean(bool),
//   LightUserData(LightUserData),
//   Integer(Integer),
//   Number(Number),
//   Vector(Vector),
//   String(String<'lua>),
//   Table(Table<'lua>),
//   Function(Function<'lua>),
//   Thread(Thread<'lua>),
//   UserData(AnyUserData<'lua>),
//   Error(Error),
// }
