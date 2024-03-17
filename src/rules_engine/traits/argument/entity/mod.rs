use anyhow::Error as AnyError;
use specs::world::Index as EntityId;

#[typetag::serde(tag = "type")]
pub trait EntityArgument {
  fn evaluate(&self) -> Result<EntityId, AnyError>;
}
