use std::collections::HashMap;

use super::{EntityId, EntityLocation};

pub struct Entities {
    locations: HashMap<EntityId, EntityLocation>
}

impl Entities {
    pub fn new() -> Self {
        Self {
            locations: HashMap::new()
        }
    }

    pub(crate) fn get_entity_location(&self, id: &EntityId) -> &EntityLocation {
        self.locations.get(id).expect("invalid entity")
    }

    pub(crate) fn get_entity_location_mut(&mut self, id: &EntityId) -> &mut EntityLocation {
        self.locations.get_mut(id).expect("invalid entity")
    }
}
