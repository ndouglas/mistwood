use anyhow::Error as AnyError;

#[typetag::serde(tag = "type")]
pub trait StringValue {
  fn evaluate(&self) -> Result<String, AnyError>;
}

value_list_trait!(StringListValue, String);
value_map_trait!(StringMapValue, String);
value_script_trait!(StringScriptValue, String);
