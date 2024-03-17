use anyhow::Error as AnyError;
use specs::world::Index as EntityId;

#[typetag::serde(tag = "type")]
pub trait EntityValue {
  fn evaluate(&self) -> Result<EntityId, AnyError>;
}

value_list_trait!(EntityListValue, EntityId);
value_map_trait!(EntityMapValue, EntityId);
