use crate::prelude::IntListValue;
use crate::prelude::IntMapValue;
use crate::prelude::IntScriptValue;
use crate::prelude::IntValue;
use anyhow::Error as AnyError;

#[typetag::serde(name = "Int")]
impl IntValue for i64 {
  fn evaluate(&self) -> Result<i64, AnyError> {
    Ok(*self)
  }
}

value_list_impl!(IntListValue, IntValue, "IntList", i64);
value_map_impl!(IntMapValue, IntValue, "IntMap", i64);
value_script_impl!(IntScriptValue, IntValue, "IntScript", i64, mlua::Value::Integer(i) => Ok(i));
