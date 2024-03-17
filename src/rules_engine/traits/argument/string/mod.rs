use anyhow::Error as AnyError;
use std::collections::HashMap;

#[typetag::serde(tag = "type")]
pub trait StringArgument {
  fn evaluate(&self) -> Result<String, AnyError>;
}

#[typetag::serde(name = "String")]
impl StringArgument for String {
  fn evaluate(&self) -> Result<String, anyhow::Error> {
    Ok(self.clone())
  }
}

impl PartialEq for dyn StringArgument {
  fn eq(&self, other: &Self) -> bool {
    self.evaluate().unwrap() == other.evaluate().unwrap()
  }
}

define_list_argument_trait_and_impl!(StringListArgument, StringArgument, "StringList", String);
define_map_argument_trait_and_impl!(StringMapArgument, StringArgument, "StringMap", String);
define_script_argument_trait_and_string_impl!(StringScriptArgument, StringArgument, "StringScript", String, mlua::Value::String(s) => Ok(s.to_str().unwrap().to_string()));
