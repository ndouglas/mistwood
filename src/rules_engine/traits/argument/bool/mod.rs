use anyhow::Error as AnyError;
use std::collections::HashMap;

#[typetag::serde(tag = "type")]
pub trait BoolArgument {
  fn evaluate(&self) -> Result<bool, AnyError>;
}

#[typetag::serde(name = "Bool")]
impl BoolArgument for bool {
  fn evaluate(&self) -> Result<bool, anyhow::Error> {
    Ok(*self)
  }
}

define_list_argument_trait_and_impl!(BoolListArgument, BoolArgument, "BoolList", bool);
define_map_argument_trait_and_impl!(BoolMapArgument, BoolArgument, "BoolMap", bool);
define_script_argument_trait_and_string_impl!(BoolScriptArgument, BoolArgument, "BoolScript", bool, mlua::Value::Boolean(b) => Ok(b));
