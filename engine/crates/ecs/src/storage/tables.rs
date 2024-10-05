use std::collections::HashMap;

use crate::component::ComponentId;

use super::{Table, TableId};

pub struct Tables {
    tables: Vec<Table>,
    map: HashMap<Box<[ComponentId]>, TableId>
}

impl Tables {
    pub fn new() -> Self {
        let mut tables = Self {
            tables: Vec::new(),
            map: HashMap::new()
        };

        tables.tables.push(Table::new());
        tables.map.insert(Box::new([]), TableId::new(0usize));

        tables
    }

    pub fn get_or_insert_table(&mut self, component_ids: &[ComponentId]) -> TableId {
        let (_, id) = self.map.raw_entry_mut().from_key(component_ids).or_insert_with(|| {
            self.tables.push(Table::new());
            (component_ids.into(), TableId::new(self.tables.len() - 1))
        });

        *id
    }

    pub(crate) fn get_table_mut(&mut self, id: TableId) -> &mut Table {
        self.tables.get_mut(id.as_usize()).expect("invalid table id")
    }

    pub(crate) fn get_table(&self, id: TableId) -> Option<&Table> {
        self.tables.get(id.as_usize())
    }
}
