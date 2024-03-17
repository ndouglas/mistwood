use crate::prelude::IntListValue;
use crate::prelude::IntOperation;
use crate::prelude::IntValue;
use anyhow::Error as AnyError;

#[derive(Derivative, Serialize, Deserialize)]
#[derivative(Debug)]
pub struct IntListLength {
  #[derivative(Debug = "ignore")]
  list: Box<dyn IntListValue>,
}

#[typetag::serde]
impl IntOperation for IntListLength {
  fn execute(&self) -> Result<Box<dyn IntValue>, AnyError> {
    Ok(Box::new(self.list.evaluate()?.len() as i64) as Box<dyn IntValue>)
  }
}

define_argument_for_operation!(IntValue, IntListLength, i64);

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_int_list_length() {
    let int_list_length = IntListLength {
      list: Box::new(vec![
        Box::new(1_i64) as Box<dyn IntValue>,
        Box::new(2_i64) as Box<dyn IntValue>,
        Box::new(3_i64) as Box<dyn IntValue>,
      ]) as Box<dyn IntListValue>,
    };
    assert_eq!(int_list_length.execute().unwrap().evaluate().unwrap(), 3);
  }

  #[test]
  fn test_int_list_length2() {
    let int_list_length = IntListLength {
      list: Box::new(vec![
        Box::new(1_i64) as Box<dyn IntValue>,
        Box::new(2_i64) as Box<dyn IntValue>,
        Box::new(3_i64) as Box<dyn IntValue>,
        Box::new(4_i64) as Box<dyn IntValue>,
      ]),
    };
    assert_eq!(int_list_length.execute().unwrap().evaluate().unwrap(), 4);
  }

  #[test]
  fn test_serde() {
    let int_list_length = &IntListLength {
      list: Box::new(vec![
        Box::new(1_i64) as Box<dyn IntValue>,
        Box::new(2_i64) as Box<dyn IntValue>,
        Box::new(3_i64) as Box<dyn IntValue>,
        Box::new(4_i64) as Box<dyn IntValue>,
      ]),
    } as &dyn IntOperation;
    let serialized = serde_yaml::to_string(int_list_length).unwrap();
    println!("{}", serialized);
    assert_eq!(
      serialized.trim(),
      r#"
type: IntListLength
list:
  type: IntList
  value:
  - type: Int
    value: 1
  - type: Int
    value: 2
  - type: Int
    value: 3
  - type: Int
    value: 4
      "#
      .trim()
    );
    let deserialized: IntListLength = serde_yaml::from_str(&serialized).unwrap();
    assert_eq!(deserialized.execute().unwrap().evaluate().unwrap(), 4);
  }
}
