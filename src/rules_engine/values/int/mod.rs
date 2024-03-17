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

#[cfg(test)]
mod tests {
  use super::*;
  use crate::prelude::IntValue;
  use crate::test::init as test_init;
  use anyhow::Error as AnyError;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_int_value() -> Result<(), AnyError> {
    test_init();
    let value = 1_i64;
    assert_eq!(value.evaluate()?, 1_i64);
    Ok(())
  }

  #[test]
  fn test_int_list_value() -> Result<(), AnyError> {
    test_init();
    let value = vec![
      Box::new(1_i64) as Box<dyn IntValue>,
      Box::new(2_i64) as Box<dyn IntValue>,
    ];
    assert_eq!(value.evaluate()?, vec![1_i64, 2_i64]);
    Ok(())
  }

  #[test]
  fn test_int_map_value() -> Result<(), AnyError> {
    test_init();
    let mut value = std::collections::HashMap::new();
    value.insert(String::from("a"), Box::new(1_i64) as Box<dyn IntValue>);
    value.insert(String::from("b"), Box::new(2_i64) as Box<dyn IntValue>);
    let expected = {
      let mut map = std::collections::HashMap::new();
      map.insert(String::from("a"), 1_i64);
      map.insert(String::from("b"), 2_i64);
      map
    };
    assert_eq!(value.evaluate()?, expected);
    Ok(())
  }

  #[test]
  fn test_serde_int_value() -> Result<(), AnyError> {
    test_init();
    let value = 1_i64;
    let serialized = serde_json::to_string(&value)?;
    assert_eq!(serialized, "1");
    let deserialized: i64 = serde_json::from_str(&serialized)?;
    assert_eq!(deserialized, value);
    Ok(())
  }

  #[test]
  fn test_serde_int_list_value() -> Result<(), AnyError> {
    test_init();
    let value = vec![1_i64, 2_i64];
    let serialized = serde_json::to_string(&value)?;
    assert_eq!(serialized, "[1,2]");
    let deserialized: Vec<i64> = serde_json::from_str(&serialized)?;
    assert_eq!(deserialized, value);
    Ok(())
  }

  #[test]
  fn test_serde_int_map_value() -> Result<(), AnyError> {
    test_init();
    let mut value = std::collections::HashMap::new();
    value.insert(String::from("a"), 1_i64);
    value.insert(String::from("b"), 2_i64);
    let serialized = serde_json::to_string(&value)?;
    let deserialized: std::collections::HashMap<String, i64> = serde_json::from_str(&serialized)?;
    assert_eq!(deserialized, value);
    Ok(())
  }
}
