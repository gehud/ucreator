use std::{any::TypeId, collections::HashMap, mem::MaybeUninit};

use uengine_any::TypeInfo;

use crate::{component::{Component, ComponentId, ComponentInfo}, entity::{Entities, EntityId}};

use super::{Column, TableId, TableRow};

pub type ColumnId = TypeId;

pub struct Table {
    columns: HashMap<ComponentId, Column>,
    components: Vec<ComponentId>
}

impl Table {
    pub fn new() -> Self {
        Self {
            columns: HashMap::new(),
            components: Vec::new()
        }
    }

    pub fn len(&self) -> usize {
        if self.columns.is_empty() {
            0usize
        } else {
            self.columns
                .get(self.components.first().unwrap())
                .unwrap()
                .len()
        }
    }

    pub fn entities(&self) -> &[EntityId] {
        if self.columns.is_empty() {
            &[]
        } else {
            self.columns
                .get(self.components.first().unwrap())
                .unwrap()
                .entities()
        }
    }

    pub(crate) fn move_entity(&mut self, entity_id: EntityId, table: &mut Table) -> TableRow {
        for (id, column) in &mut self.columns {
            let data = column.remove_data(entity_id).unwrap();
            table.columns.get_mut(id).unwrap().insert_data(entity_id, data.as_slice());
        }

        TableRow::new(table.len() - 1)
    }

    pub fn has_column(&self, id: &ComponentId) -> bool {
        self.columns.contains_key(id)
    }

    pub(crate) fn get_column(&self, component_id: ComponentId) -> Option<&Column> {
        self.columns.get(&component_id)
    }

    pub(crate) fn get_column_mut(&mut self, component_id: ComponentId) -> Option<&mut Column> {
        self.columns.get_mut(&component_id)
    }

    pub(crate) fn get_or_create_column(&mut self, component_info: &ComponentInfo) -> &mut Column {
        self.columns.entry(component_info.id()).or_insert_with(|| {
            match self.components.binary_search(&component_info.id()) {
                Ok(_) => panic!("wrong type insersion"),
                Err(index) => self.components.insert(index, component_info.id()),
            };

            Column::from_info(component_info)
        })
    }

    pub fn components(&self) -> &[ComponentId] {
        &self.components
    }
}
