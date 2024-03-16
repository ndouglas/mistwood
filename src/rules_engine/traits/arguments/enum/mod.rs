use crate::rules_engine::traits::argument::Argument;
use anyhow::Error as AnyError;
use std::collections::HashMap;

#[typetag::serde(tag = "type")]
pub trait EnumArgument: Argument {
  fn value(&self) -> Result<&str, AnyError>;
}

define_list_argument_trait_and_impl!(EnumListArgument, EnumArgument, "EnumList");
define_map_argument_trait_and_impl!(EnumMapArgument, EnumArgument, "EnumMap");
