use anyhow::Error as AnyError;
use specs::prelude::*;
use std::collections::HashMap;

#[typetag::serde(tag = "type")]
pub trait EntityArgument {
  fn value(&self) -> Result<Entity, AnyError>;
}

define_list_argument_trait_and_impl!(EntityListArgument, EntityArgument, "EntityList");
define_map_argument_trait_and_impl!(EntityMapArgument, EntityArgument, "EntityMap");
