use anyhow::Error as AnyError;

#[typetag::serde(tag = "type")]
pub trait EnumValue {
  fn evaluate(&self) -> Result<String, AnyError>;
}

value_list_trait!(EnumListValue, String);
value_map_trait!(EnumMapValue, String);
