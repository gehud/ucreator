use std::collections::{hash_map::Entry, HashMap};

use uengine_any::AnyVec;

use crate::{component::Component, Entity};

pub struct Column {
    components: AnyVec,
    indices: HashMap<Entity, usize>
}

impl Column {
    pub fn new<C: Component>() -> Self {
        Self {
            components: AnyVec::new::<C>(),
            indices: HashMap::new()
        }
    }

    pub fn insert<C: Component>(&mut self, entity: Entity, component: C) -> Option<C> {
        match self.indices.entry(entity) {
            Entry::Occupied(entry) => {
                let index = *entry.get();
                let old_value = self.components.remove(index);
                self.components.insert(index, component);
                Some(old_value)
            },
            Entry::Vacant(entry) => {
                self.components.push(component);
                entry.insert(self.components.len() - 1);
                None
            },
        }
    }

    pub fn remove<C: Component>(&mut self, entity: &Entity) -> Option<C> {
        let index = self.indices.remove(entity)?;
        Some(self.components.remove::<C>(index))
    }

    pub fn get<C: Component>(&self, entity: &Entity) -> Option<&C> {
        let index = *self.indices.get(entity)?;
        self.components.get(index)
    }

    pub fn get_mut<C: Component>(&mut self, entity: &Entity) -> Option<&mut C> {
        let index = *self.indices.get(entity)?;
        self.components.get_mut(index)
    }
}
