use anyhow::Error as AnyError;
use std::collections::HashMap;

#[typetag::serde(tag = "type")]
pub trait StringArgument {
  fn value(&self) -> Result<String, AnyError>;
}

#[typetag::serde(name = "String")]
impl StringArgument for String {
  fn value(&self) -> Result<String, anyhow::Error> {
    Ok(self.clone())
  }
}

impl PartialEq for dyn StringArgument {
  fn eq(&self, other: &Self) -> bool {
    self.value().unwrap() == other.value().unwrap()
  }
}

#[derive(Derivative, Deserialize, Serialize)]
pub struct StringScriptArgument {
  script: String,
}

#[typetag::serde(name = "StringScript")]
impl StringArgument for StringScriptArgument {
  fn value(&self) -> Result<String, AnyError> {
    use mlua::{Function, Lua, Value};
    let lua = Lua::new();
    let _globals = lua.globals();
    let function: Function = lua.load(&self.script).eval()?;
    let result = function.call(())?;
    match result {
      Value::String(value) => Ok(value.to_str()?.to_string()),
      _ => Err(AnyError::msg(format!(
        "Expected {} but got {:?}",
        stringify!($return_type),
        result
      ))),
    }
  }
}

define_list_argument_trait_and_impl!(StringListArgument, StringArgument, "StringList");
define_map_argument_trait_and_impl!(StringMapArgument, StringArgument, "StringMap");
