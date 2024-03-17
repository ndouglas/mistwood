use anyhow::Error as AnyError;
use mlua::{Function, Lua};
use std::collections::HashMap;

#[typetag::serde(tag = "type")]
pub trait ScriptArgument {
  fn get_context(&self) -> &Lua;
  fn get_script(&self) -> &str;
  fn evaluate(&self) -> Result<Function, AnyError> {
    let lua = self.get_context();
    let script = self.get_script();
    Ok(lua.load(script).eval()?)
  }
}

define_list_argument_trait_and_impl!(ScriptListArgument, ScriptArgument, "ScriptList");
define_map_argument_trait_and_impl!(ScriptMapArgument, ScriptArgument, "ScriptMap");
