use std::{
    collections::{
        hash_map::Entry,
        HashMap
    },
    mem::MaybeUninit, vec::Drain
};

use uengine_any::{AnyVec, TypeInfo};

use crate::{component::{Component, ComponentInfo}, entity::EntityId};

use super::TableRow;

pub struct Column {
    components: AnyVec,
    indices: HashMap<EntityId, usize>,
    entities: Vec<EntityId>
}

impl Column {
    pub fn new<C: Component>() -> Self {
        Self {
            components: AnyVec::new::<C>(),
            indices: HashMap::new(),
            entities: Vec::new()
        }
    }

    pub(super) fn from_info(component_info: &ComponentInfo) -> Self {
        Self {
            components: AnyVec::from_info(*component_info.type_info()),
            indices: HashMap::new(),
            entities: Vec::new()
        }
    }

    pub fn entities(&self) -> &[EntityId] {
        &self.entities
    }

    pub fn len(&self) -> usize {
        self.entities.len()
    }

    pub(crate) fn allocate_row(&mut self) -> usize {
        self.components.allocate();
        self.components.len() - 1
    }

    pub(crate) fn write_component(&mut self, table_row: TableRow, data: &[MaybeUninit<u8>]) {
        self.components.write_data(table_row.as_usize(), data);
    }

    pub(crate) fn insert_data(&mut self, entity_id: EntityId, data: &[MaybeUninit<u8>]) {
        match self.indices.entry(entity_id) {
            Entry::Occupied(entry) => {
                let index = *entry.get();
                self.components.remove_data(index);
                self.components.insert_data(index, data);
            },
            Entry::Vacant(entry) => {
                self.components.push_data(data);
                entry.insert(self.components.len() - 1);
                self.entities.push(entity_id);
            },
        };
    }

    pub fn insert<C: Component>(&mut self, entity: EntityId, component: C) -> Option<C> {
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
                self.entities.push(entity);
                None
            },
        }
    }

    pub fn remove<C: Component>(&mut self, entity: EntityId) -> Option<C> {
        let index = self.indices.remove(&entity)?;
        Some(self.components.remove::<C>(index))
    }

    pub(crate) fn remove_data(&mut self, entity: EntityId) -> Option<Drain<MaybeUninit<u8>>> {
        let index = self.indices.remove(&entity)?;
        Some(self.components.remove_data(index))
    }

    pub fn get<C: Component>(&self, entity: &EntityId) -> Option<&C> {
        let index = *self.indices.get(entity)?;
        self.components.get(index)
    }

    pub fn get_mut<C: Component>(&mut self, entity: &EntityId) -> Option<&mut C> {
        let index = *self.indices.get(entity)?;
        self.components.get_mut(index)
    }
}
