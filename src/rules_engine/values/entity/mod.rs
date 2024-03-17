use crate::prelude::EntityValue;
use anyhow::Error as AnyError;
use specs::world::Index as EntityId;
use std::collections::HashMap;

define_list_argument_trait_and_impl!(EntityListValue, EntityValue, "EntityList", EntityId);
define_map_argument_trait_and_impl!(EntityMapValue, EntityValue, "EntityMap", EntityId);
