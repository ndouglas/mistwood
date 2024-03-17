use crate::prelude::BoolListValue;
use crate::prelude::BoolMapValue;
use crate::prelude::BoolScriptValue;
use crate::rules_engine::prelude::BoolValue;
use anyhow::Error as AnyError;

#[typetag::serde(name = "Bool")]
impl BoolValue for bool {
  fn evaluate(&self) -> Result<bool, AnyError> {
    Ok(*self)
  }
}

value_list_impl!(BoolListValue, BoolValue, "BoolList", bool);
value_map_impl!(BoolMapValue, BoolValue, "BoolMap", bool);
value_script_impl!(BoolScriptValue, BoolValue, "BoolScript", bool, mlua::Value::Boolean(b) => Ok(b));
