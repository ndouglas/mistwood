use anyhow::Error as AnyError;

#[typetag::serde(tag = "type")]
pub trait IntValue {
  fn evaluate(&self) -> Result<i64, AnyError>;
}

value_list_trait!(IntListValue, i64);
value_map_trait!(IntMapValue, i64);
value_script_trait!(IntScriptValue, i64);
