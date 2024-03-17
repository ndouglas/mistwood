use anyhow::Error as AnyError;

#[typetag::serde(tag = "type")]
pub trait FloatValue {
  fn evaluate(&self) -> Result<f64, AnyError>;
}

value_list_trait!(FloatListValue, f64);
value_map_trait!(FloatMapValue, f64);
value_script_trait!(FloatScriptValue, f64);
