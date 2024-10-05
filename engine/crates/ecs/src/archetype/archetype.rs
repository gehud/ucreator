use std::{any::TypeId, collections::HashMap};

use crate::{component::{ComponentId, ComponentInfo, StoragePolicy}, storage::{Columns, Storages, Table, TableId, Tables}, world::World};

use super::{archetypes::ArchetypeGraphNode, ArchetypeId, Archetypes};

#[derive(Clone)]
pub struct Archetype {
    id: ArchetypeId,
    table_id: TableId,
    column_ids: Vec<ComponentId>,
    components: Vec<ComponentId>
}

impl Archetype {
    pub(crate) fn new(id: ArchetypeId, table_id: TableId, column_ids: Vec<ComponentId>, component_ids: Vec<ComponentId>) -> Self {
        Self {
            id,
            table_id,
            column_ids,
            components: component_ids
        }
    }

    pub fn id(&self) -> ArchetypeId {
        self.id
    }

    pub fn table_id(&self) -> TableId {
        self.table_id
    }

    pub fn column_ids(&self) -> &[ComponentId] {
        &self.column_ids
    }

    pub fn component_ids(&self) -> &[ComponentId] {
        &self.components
    }

    pub(crate) fn get_or_create_extended(
        mut self,
        mut alteration: impl FnMut(&mut Archetype, &ComponentInfo),
        archetypes: &mut Archetypes,
        component_info: ComponentInfo
    ) -> ArchetypeId {
        match self.components.binary_search(&component_info.type_info().id()) {
            Ok(_) => panic!("wrong type insertion"),
            Err(pos) => {
                self.components.insert(pos, *component_info.type_info().id());
            },
        };

        match archetypes.get_archetype_meta(&self.components) {
            Some(meta) => meta.id,
            None => {
                let meta = archetypes.get_archetype_meta(&self.components).unwrap();

                alteration(&mut self, &component_info);

                self.id = ArchetypeId::new(archetypes.archetypes.len());
                archetypes.archetypes.push(self);

                archetypes.graph.push(ArchetypeGraphNode {
                    archetype_id: ArchetypeId::new(archetypes.archetypes.len() - 1),
                    parent: meta.graph_index,
                    type_id: *component_info.type_info().id(),
                    children: HashMap::new()
                });

                let graph_index = archetypes.graph.len() - 1;
                let node = archetypes.get_archetype_graph_node_mut(meta.graph_index).unwrap();
                node.children.insert(*component_info.type_info().id(), graph_index);
                ArchetypeId::new(archetypes.archetypes.len() - 1)
            },
        }
    }
}
