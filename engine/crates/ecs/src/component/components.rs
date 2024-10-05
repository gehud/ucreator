use std::{any::TypeId, collections::HashMap};

use super::{Component, ComponentId, ComponentInfo};

pub struct Components {
    components: Vec<ComponentInfo>,
    ids: HashMap<TypeId, ComponentId>
}

impl Components {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
            ids: HashMap::new()
        }
    }

    pub fn get_component_id<C: Component>(&self) -> Option<ComponentId> {
        self.ids.get(&TypeId::of::<C>()).cloned()
    }

    pub fn get_component_info(&self, id: ComponentId) -> &ComponentInfo {
        self.components.get(id.as_usize()).expect("invalid component id")
    }
}
