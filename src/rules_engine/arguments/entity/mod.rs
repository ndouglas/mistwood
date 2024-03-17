use crate::prelude::EntityArgument;
use anyhow::Error as AnyError;
use specs::world::Index as EntityId;
use std::collections::HashMap;

define_list_argument_trait_and_impl!(EntityListArgument, EntityArgument, "EntityList", EntityId);
define_map_argument_trait_and_impl!(EntityMapArgument, EntityArgument, "EntityMap", EntityId);
