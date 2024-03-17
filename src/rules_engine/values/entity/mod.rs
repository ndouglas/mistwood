use crate::prelude::EntityListValue;
use crate::prelude::EntityMapValue;
use crate::prelude::EntityValue;
use anyhow::Error as AnyError;
use specs::world::Index as EntityId;

#[typetag::serde(name = "EntityValue")]
impl EntityValue for EntityId {
  fn evaluate(&self) -> Result<EntityId, AnyError> {
    Ok(*self)
  }
}

value_list_impl!(EntityListValue, EntityValue, "EntityList", EntityId);
value_map_impl!(EntityMapValue, EntityValue, "EntityMap", EntityId);

#[cfg(test)]
mod tests {
  use super::*;
  use crate::prelude::EntityValue;
  use crate::test::init as test_init;
  use anyhow::Error as AnyError;
  use std::collections::HashMap;

  #[test]
  fn test_entity_value() -> Result<(), AnyError> {
    test_init();
    let value = 1 as EntityId;
    assert_eq!(value.evaluate()?, 1 as EntityId);
    Ok(())
  }

  #[test]
  fn test_entity_list_value() -> Result<(), AnyError> {
    test_init();
    let value = vec![
      Box::new(1 as EntityId) as Box<dyn EntityValue>,
      Box::new(2 as EntityId) as Box<dyn EntityValue>,
    ];
    assert_eq!(value.evaluate()?, vec![1, 2]);
    Ok(())
  }

  #[test]
  fn test_entity_map_value() -> Result<(), AnyError> {
    test_init();
    let mut value = HashMap::new();
    value.insert(String::from("a"), Box::new(1 as EntityId) as Box<dyn EntityValue>);
    value.insert(String::from("b"), Box::new(2 as EntityId) as Box<dyn EntityValue>);
    let expected = {
      let mut map = HashMap::new();
      map.insert(String::from("a"), 1 as EntityId);
      map.insert(String::from("b"), 2 as EntityId);
      map
    };
    assert_eq!(value.evaluate()?, expected);
    Ok(())
  }
}
