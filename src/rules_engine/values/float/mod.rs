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

#[cfg(test)]
mod tests {
  use super::*;
  use crate::prelude::FloatValue;
  use crate::test::init as test_init;
  use anyhow::Error as AnyError;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_float_value() -> Result<(), AnyError> {
    test_init();
    let value = 1.0;
    assert_eq!(value.evaluate()?, 1.0);
    Ok(())
  }

  #[test]
  fn test_float_list_value() -> Result<(), AnyError> {
    test_init();
    let value = vec![
      Box::new(1.0) as Box<dyn FloatValue>,
      Box::new(2.0) as Box<dyn FloatValue>,
    ];
    assert_eq!(value.evaluate()?, vec![1.0, 2.0]);
    Ok(())
  }

  #[test]
  fn test_float_map_value() -> Result<(), AnyError> {
    test_init();
    let mut value = std::collections::HashMap::new();
    value.insert(String::from("a"), Box::new(1.0) as Box<dyn FloatValue>);
    value.insert(String::from("b"), Box::new(2.0) as Box<dyn FloatValue>);
    let expected = {
      let mut map = std::collections::HashMap::new();
      map.insert(String::from("a"), 1.0);
      map.insert(String::from("b"), 2.0);
      map
    };
    assert_eq!(value.evaluate()?, expected);
    Ok(())
  }

  #[test]
  fn test_serde_float_value() -> Result<(), AnyError> {
    test_init();
    let value = 1.0;
    let serialized = serde_yaml::to_string(&value)?;
    let deserialized: f64 = serde_yaml::from_str(&serialized)?;
    assert_eq!(value, deserialized);
    Ok(())
  }
}
