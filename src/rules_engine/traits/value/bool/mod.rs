use anyhow::Error as AnyError;

#[typetag::serde(tag = "type")]
pub trait BoolValue {
  fn evaluate(&self) -> Result<bool, AnyError>;
}

value_list_trait!(BoolListValue, bool);
value_map_trait!(BoolMapValue, bool);
value_script_trait!(BoolScriptValue, bool);
