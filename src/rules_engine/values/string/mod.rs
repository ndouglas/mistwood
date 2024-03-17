use crate::prelude::StringListValue;
use crate::prelude::StringMapValue;
use crate::prelude::StringScriptValue;
use crate::prelude::StringValue;
use anyhow::Error as AnyError;

#[typetag::serde(name = "String")]
impl StringValue for String {
  fn evaluate(&self) -> Result<String, AnyError> {
    Ok(self.clone())
  }
}

impl PartialEq for dyn StringValue {
  fn eq(&self, other: &Self) -> bool {
    self.evaluate().unwrap() == other.evaluate().unwrap()
  }
}

value_list_impl!(StringListValue, StringValue, "StringList", String);
value_map_impl!(StringMapValue, StringValue, "StringMap", String);
value_script_impl!(StringScriptValue, StringValue, "StringScript", String, mlua::Value::String(s) => Ok(s.to_str().unwrap().to_string()));
