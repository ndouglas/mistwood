use crate::prelude::FloatListValue;
use crate::prelude::FloatMapValue;
use crate::prelude::FloatScriptValue;
use crate::prelude::FloatValue;

#[typetag::serde(name = "Float")]
impl FloatValue for f64 {
  fn evaluate(&self) -> Result<f64, anyhow::Error> {
    Ok(*self)
  }
}

value_list_impl!(FloatListValue, FloatValue, "FloatList", f64);
value_map_impl!(FloatMapValue, FloatValue, "FloatMap", f64);
value_script_impl!(FloatScriptValue, FloatValue, "FloatScript", f64, mlua::Value::Number(b) => Ok(b));
