use super::{
    Columns, Tables
};

pub struct Storages {
    tables: Tables,
    columns: Columns
}

impl Storages {
    pub fn new() -> Self {
        Self {
            tables: Tables::new(),
            columns: Columns::new()
        }
    }

    pub fn tables(&self) -> &Tables {
        &self.tables
    }

    pub fn tables_mut(&mut self) -> &mut Tables {
        &mut self.tables
    }

    pub fn columns(&self) -> &Columns {
        &self.columns
    }

    pub fn columns_mut(&mut self) -> &mut Columns {
        &mut self.columns
    }
}
