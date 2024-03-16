use crate::prelude::IntArgument;
use crate::prelude::IntListArgument;
use crate::prelude::IntOperation;
use anyhow::Error as AnyError;

#[derive(Derivative, Serialize, Deserialize)]
#[derivative(Debug)]
pub struct IntListLength {
  #[derivative(Debug = "ignore")]
  list: Box<dyn IntListArgument>,
}

#[typetag::serde]
impl IntOperation for IntListLength {
  fn execute(&self) -> Result<Box<dyn IntArgument>, AnyError> {
    Ok(Box::new(self.list.value()?.len() as i64) as Box<dyn IntArgument>)
  }
}

#[typetag::serde]
impl IntArgument for IntListLength {
  fn value(&self) -> Result<i64, AnyError> {
    self.execute()?.value()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_int_list_length() {
    let int_list_length = IntListLength {
      list: Box::new(vec![
        Box::new(1_i64) as Box<dyn IntArgument>,
        Box::new(2_i64) as Box<dyn IntArgument>,
        Box::new(3_i64) as Box<dyn IntArgument>,
      ]) as Box<dyn IntListArgument>,
    };
    assert_eq!(int_list_length.execute().unwrap().value().unwrap(), 3);
  }

  #[test]
  fn test_int_list_length2() {
    let int_list_length = IntListLength {
      list: Box::new(vec![
        Box::new(1_i64) as Box<dyn IntArgument>,
        Box::new(2_i64) as Box<dyn IntArgument>,
        Box::new(3_i64) as Box<dyn IntArgument>,
        Box::new(4_i64) as Box<dyn IntArgument>,
      ]),
    };
    assert_eq!(int_list_length.execute().unwrap().value().unwrap(), 4);
  }

  #[test]
  fn test_serde() {
    let int_list_length = &IntListLength {
      list: Box::new(vec![
        Box::new(1_i64) as Box<dyn IntArgument>,
        Box::new(2_i64) as Box<dyn IntArgument>,
        Box::new(3_i64) as Box<dyn IntArgument>,
        Box::new(4_i64) as Box<dyn IntArgument>,
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
    assert_eq!(deserialized.execute().unwrap().value().unwrap(), 4);
  }
}
