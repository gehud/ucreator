use crate::storage::{ColumnId, TableId};

pub struct Archetype {
    table_id: TableId,
    column_ids: Vec<ColumnId>
}
