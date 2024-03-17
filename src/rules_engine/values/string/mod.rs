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

#[cfg(test)]
mod tests {
  use super::*;
  use crate::prelude::StringValue;
  use crate::test::init as test_init;
  use anyhow::Error as AnyError;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_string_value() -> Result<(), AnyError> {
    test_init();
    let value = String::from("a");
    assert_eq!(StringValue::evaluate(&value)?, String::from("a"));
    Ok(())
  }

  #[test]
  fn test_string_list_value() -> Result<(), AnyError> {
    test_init();
    let value = vec![
      Box::new(String::from("a")) as Box<dyn StringValue>,
      Box::new(String::from("b")) as Box<dyn StringValue>,
    ];
    assert_eq!(
      StringListValue::evaluate(&value)?,
      vec![String::from("a"), String::from("b")]
    );
    Ok(())
  }

  #[test]
  fn test_string_map_value() -> Result<(), AnyError> {
    test_init();
    let mut value = std::collections::HashMap::new();
    value.insert(String::from("a"), Box::new(String::from("a")) as Box<dyn StringValue>);
    value.insert(String::from("b"), Box::new(String::from("b")) as Box<dyn StringValue>);
    let expected = {
      let mut map = std::collections::HashMap::new();
      map.insert(String::from("a"), String::from("a"));
      map.insert(String::from("b"), String::from("b"));
      map
    };
    assert_eq!(value.evaluate()?, expected);
    Ok(())
  }

  #[test]
  fn test_serde_string_value() -> Result<(), AnyError> {
    test_init();
    let value = String::from("a");
    let serialized = serde_json::to_string(&value)?;
    assert_eq!(serialized, "\"a\"");
    let deserialized: String = serde_json::from_str(&serialized)?;
    assert_eq!(deserialized, String::from("a"));
    Ok(())
  }

  #[test]
  fn test_serde_string_list_value() -> Result<(), AnyError> {
    test_init();
    let value = vec![String::from("a"), String::from("b")];
    let serialized = serde_json::to_string(&value)?;
    assert_eq!(serialized, "[\"a\",\"b\"]");
    let deserialized: Vec<String> = serde_json::from_str(&serialized)?;
    assert_eq!(deserialized, value);
    Ok(())
  }

  #[test]
  fn test_serde_string_map_value() -> Result<(), AnyError> {
    test_init();
    let mut value = std::collections::HashMap::new();
    value.insert(String::from("a"), String::from("a"));
    value.insert(String::from("b"), String::from("b"));
    let serialized = serde_json::to_string(&value)?;
    let deserialized: std::collections::HashMap<String, String> = serde_json::from_str(&serialized)?;
    assert_eq!(deserialized, value);
    Ok(())
  }

  #[test]
  fn test_string_script_value() -> Result<(), AnyError> {
    test_init();
    let value = String::from("return \"a\"");
    assert_eq!(StringScriptValue::evaluate(&value)?, String::from("a"));
    Ok(())
  }
}
