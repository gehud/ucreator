use crate::{
    archetype::Archetypes, component::{
        Component,
        StoragePolicy
    }, Entity
};

use super::{
    Table, Tables
};

pub struct Storage {
    archetypes: Archetypes,
    dense: Tables,
    sparse: Table
}

impl Storage {
    pub fn new() -> Self {
        Self {
            archetypes: Archetypes::new(),
            dense: Tables::new(),
            sparse: Table::new()
        }
    }

    pub(crate) fn insert_empty_entity(&mut self, entity: Entity) {
        self.dense.insert_empty_entity(entity);
    }

    pub fn insert<C: Component>(&mut self, entity: Entity, component: C) -> Option<C> {
        match C::STORAGE_POLICY {
            StoragePolicy::Dense => {
                self.dense.insert(entity, component)
            },
            StoragePolicy::Sparse => {
                self.sparse.insert(entity, component)
            },
        }
    }

    pub fn get<C: Component>(&self, entity: &Entity) -> Option<&C> {
        match C::STORAGE_POLICY {
            StoragePolicy::Dense => {
                self.dense.get::<C>(entity)
            },
            StoragePolicy::Sparse => {
                self.sparse.get::<C>(entity)
            },
        }
    }

    pub fn get_mut<C: Component>(&mut self, entity: &Entity) -> Option<&mut C> {
        match C::STORAGE_POLICY {
            StoragePolicy::Dense => {
                self.dense.get_mut::<C>(entity)
            },
            StoragePolicy::Sparse => {
                self.sparse.get_mut::<C>(entity)
            },
        }
    }

    pub fn remove<C: Component>(&mut self, entity: &Entity) -> Option<C> {
        match C::STORAGE_POLICY {
            StoragePolicy::Dense => {
                self.dense.remove(entity)
            },
            StoragePolicy::Sparse => {
                self.sparse.remove(entity)
            },
        }
    }
}
