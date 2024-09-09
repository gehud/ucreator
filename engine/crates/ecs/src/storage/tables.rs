use std::{any::TypeId, collections::HashMap};

use crate::{component::Component, Entity};

use super::Table;

pub type TableId = usize;

pub struct Tables {
    tables: Vec<Table>,
    entities: HashMap<Entity, TableId>,
    tables_ids: HashMap<Box<[TypeId]>, TableId>
}

impl Tables {
    pub fn new() -> Self {
        let mut table = Self {
            tables: Vec::new(),
            entities: HashMap::new(),
            tables_ids: HashMap::new()
        };

        table.tables.push(Table::new());
        table.tables_ids.insert(Box::new([TypeId::of::<()>()]), 0usize);

        table
    }

    pub(crate) fn insert_empty_entity(&mut self, entity: Entity) {
        self.entities.insert(entity, 0usize);
    }

    pub fn insert<C: Component>(&mut self, entity: Entity, component: C) -> Option<C> {
        let old_index = *self.entities.get(&entity)
            .expect("the entity must initially be at least in an empty table");
        let old_table = self.tables.get(old_index).unwrap();
        // TODO: replace with unsized_locals feature https://github.com/rust-lang/rust/issues/48055
        let types = [old_table.types(), &[TypeId::of::<C>()]].concat();
        let (index, table) = self.get_or_insert(&types);
        let result = table.insert(entity, component);
        *self.entities.get_mut(&entity).unwrap() = index;
        result
    }

    pub fn get<C: Component>(&self, entity: &Entity) -> Option<&C> {
        let index = *self.entities.get(entity)?;
        self.tables.get(index).unwrap().get(entity)
    }

    pub fn get_mut<C: Component>(&mut self, entity: &Entity) -> Option<&mut C> {
        let index = *self.entities.get(entity)?;
        self.tables.get_mut(index).unwrap().get_mut(entity)
    }

    pub fn remove<C: Component>(&mut self, entity: &Entity) -> Option<C> {
        let index = *self.entities.get(entity)?;
        self.tables.get_mut(index).unwrap().remove(entity)
    }

    fn get_or_insert(&mut self, types: &[TypeId]) -> (TableId, &mut Table) {
        let (_, index) = self.tables_ids.raw_entry_mut().from_key(types).or_insert_with(|| {
            self.tables.push(Table::new());
            (types.into(), self.tables.len() - 1)
        });

        (*index, self.tables.get_mut(*index).unwrap())
    }
}
