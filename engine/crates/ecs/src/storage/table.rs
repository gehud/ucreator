use std::{any::TypeId, collections::HashMap};

use crate::{component::Component, Entity};

use super::Column;

pub type ColumnId = TypeId;

pub struct Table {
    columns: HashMap<TypeId, Column>,
    types: Vec<TypeId>
}

impl Table {
    pub fn new() -> Self {
        Self {
            columns: HashMap::new(),
            types: Vec::new()
        }
    }

    pub fn column<C: Component>(&self) -> Option<&Column> {
        self.columns.get(&TypeId::of::<C>())
    }

    pub fn column_mut<C: Component>(&mut self) -> Option<&mut Column> {
        self.columns.get_mut(&TypeId::of::<C>())
    }

    pub fn types(&self) -> &[TypeId] {
        &self.types
    }

    pub fn insert<C: Component>(&mut self, entity: Entity, component: C) -> Option<C> {
        let column = self.columns.entry(TypeId::of::<C>()).or_insert_with(|| {
            let type_id = TypeId::of::<C>();
            match self.types.binary_search(&type_id) {
                Ok(_) => panic!("wrong type insersion"),
                Err(index) => self.types.insert(index, type_id),
            };

            Column::new::<C>()
        });

        column.insert(entity, component)
    }

    pub fn remove<C: Component>(&mut self, entity: &Entity) -> Option<C> {
        let column = self.columns.get_mut(&TypeId::of::<C>())?;
        column.remove(entity)
    }

    pub fn get<C: Component>(&self, entity: &Entity) -> Option<&C> {
        let column = self.columns.get(&TypeId::of::<C>())?;
        column.get(entity)
    }

    pub fn get_mut<C: Component>(&mut self, entity: &Entity) -> Option<&mut C> {
        let column = self.columns.get_mut(&TypeId::of::<C>())?;
        column.get_mut(entity)
    }
}
