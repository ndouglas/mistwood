use crate::prelude::EntityListValue;
use crate::prelude::EntityMapValue;
use crate::prelude::EntityValue;
use specs::world::Index as EntityId;

value_list_impl!(EntityListValue, EntityValue, "EntityList", EntityId);
value_map_impl!(EntityMapValue, EntityValue, "EntityMap", EntityId);
