use crate::rules_engine::prelude::BoolValue;
use anyhow::Error as AnyError;
use std::collections::HashMap;

#[typetag::serde(name = "Bool")]
impl BoolValue for bool {
  fn evaluate(&self) -> Result<bool, AnyError> {
    Ok(*self)
  }
}

define_list_argument_trait_and_impl!(BoolListValue, BoolValue, "BoolList", bool);
define_map_argument_trait_and_impl!(BoolMapValue, BoolValue, "BoolMap", bool);
define_script_argument_trait_and_string_impl!(BoolScriptValue, BoolValue, "BoolScript", bool, mlua::Value::Boolean(b) => Ok(b));
