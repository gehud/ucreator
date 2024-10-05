use std::collections::HashMap;

use crate::component::{ComponentId, ComponentInfo};

use super::Column;

pub struct Columns {
    columns: HashMap<ComponentId, Column>
}

impl Columns {
    pub fn new() -> Self {
        Self {
            columns: HashMap::new()
        }
    }

    pub fn has_column(&self, id: &ComponentId) -> bool {
        self.columns.contains_key(id)
    }

    pub(crate) fn get_or_create_column(&mut self, component_info: &ComponentInfo) -> &mut Column {
        self.columns.entry(component_info.id()).or_insert_with(|| {
            Column::from_info(component_info)
        })
    }

    pub(crate) fn get_column(&self, id: ComponentId) -> Option<&Column> {
        self.columns.get(&id)
    }

    pub(crate) fn get_column_mut(&mut self, id: ComponentId) -> Option<&mut Column> {
        self.columns.get_mut(&id)
    }
}
