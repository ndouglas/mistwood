use anyhow::Error as AnyError;
use mlua::{Function, Lua};

#[typetag::serde(tag = "type")]
pub trait ScriptArgument {
  fn get_context(&self) -> &Lua;
  fn get_script(&self) -> &str;
  fn value(&self) -> Result<Function, AnyError> {
    let lua = self.get_context();
    let script = self.get_script();
    Ok(lua.load(script).eval()?)
  }
}
