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

#[cfg(test)]
mod tests {
  use super::*;
  use crate::prelude::BoolValue;
  use crate::test::init as test_init;
  use anyhow::Error as AnyError;
  use pretty_assertions::assert_eq;
  use std::collections::HashMap;

  #[test]
  fn test_bool_value() -> Result<(), AnyError> {
    test_init();
    let value = true;
    assert_eq!(value.evaluate()?, true);
    Ok(())
  }

  #[test]
  fn test_bool_list_value() -> Result<(), AnyError> {
    test_init();
    let value = vec![
      Box::new(true) as Box<dyn BoolValue>,
      Box::new(false) as Box<dyn BoolValue>,
    ];
    assert_eq!(value.evaluate()?, vec![true, false]);
    Ok(())
  }

  #[test]
  fn test_bool_map_value() -> Result<(), AnyError> {
    test_init();
    let mut value = HashMap::new();
    value.insert(String::from("a"), Box::new(true) as Box<dyn BoolValue>);
    value.insert(String::from("b"), Box::new(false) as Box<dyn BoolValue>);
    let expected = {
      let mut map = HashMap::new();
      map.insert(String::from("a"), true);
      map.insert(String::from("b"), false);
      map
    };
    assert_eq!(value.evaluate()?, expected);
    Ok(())
  }

  #[test]
  fn test_bool_script_value() -> Result<(), AnyError> {
    test_init();
    let value = String::from("return true");
    assert_eq!(value.evaluate()?, true);
    Ok(())
  }

  #[test]
  fn test_serde_bool_value() -> Result<(), AnyError> {
    test_init();
    let value = true;
    let serialized = serde_yaml::to_string(&value)?;
    let deserialized: bool = serde_yaml::from_str(&serialized)?;
    assert_eq!(value, deserialized);
    Ok(())
  }
}
