use crate::{archetype::ArchetypeId, storage::TableRow};

#[derive(Clone, Copy)]
pub struct EntityLocation {
    archetype_id: ArchetypeId,
    table_row: TableRow
}

impl EntityLocation {
    pub(crate) fn new(archetype_id: ArchetypeId, table_row: TableRow) -> Self {
        Self {
            archetype_id,
            table_row
        }
    }

    pub fn archetype_id(&self) -> ArchetypeId {
        self.archetype_id
    }

    pub fn table_row(&self) -> TableRow {
        self.table_row
    }
}
