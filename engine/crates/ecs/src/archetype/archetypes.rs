use std::{any::TypeId, collections::HashMap, hash::Hash, ops::{Index, IndexMut}, usize};

use crate::{bundle::Bundle, component::{ComponentId, Components, StoragePolicy}, entity::EntityId, storage::{Storages, Table, TableId}};

use super::{Archetype, ArchetypeId};

pub struct ArchetypeGraphNode {
    pub type_id: TypeId,
    pub archetype_id: ArchetypeId,
    pub parent: usize,
    pub children: HashMap<TypeId, usize>
}

#[derive(Clone, Copy)]
struct ArchetypeMeta {
    pub id: ArchetypeId,
    pub graph_index: usize
}

#[derive(Hash, PartialEq, Eq)]
struct ArchetypeKey {
    table_id: TableId,
    column_ids: Box<[ComponentId]>
}

pub struct Archetypes {
    archetypes: Vec<Archetype>,
    meta: HashMap<ArchetypeKey, ArchetypeMeta>,
    graph: Vec<ArchetypeGraphNode>,
    entities: HashMap<EntityId, ArchetypeId>
}

impl Archetypes {
    pub fn new() -> Self {
        let mut archetypes = Self {
            archetypes: Vec::new(),
            meta: HashMap::new(),
            graph: Vec::new(),
            entities: HashMap::new()
        };

        archetypes.graph.push(ArchetypeGraphNode {
            archetype_id: ArchetypeId::new(0usize),
            type_id: TypeId::of::<()>(),
            parent: usize::MAX,
            children: HashMap::new()
        });

        archetypes.archetypes.push(Archetype::new(
            ArchetypeId::new(0usize),
            TableId::new(0usize),
            Vec::new(),
            Vec::new()
        ));

        archetypes
    }

    pub(crate) fn get_or_insert_archetype(&mut self, table_id: TableId, column_ids: Vec<ComponentId>, component_ids: Vec<ComponentId>) -> ArchetypeId {
        let key = ArchetypeKey {
            table_id,
            column_ids: column_ids.into_boxed_slice()
        };

        self.meta.entry(key).or_insert_with(|| {
            let archetype = Archetype::new(
                ArchetypeId::new(self.archetypes.len()),
                table_id,
                column_ids,
                component_ids
            );

            self.archetypes.push(archetype);

            ArchetypeMeta {

            }
        })
        .id
    }

    pub(crate) fn insert_empty_entity(&mut self, entity: EntityId) {
        self.entities.insert(entity, ArchetypeId::new(0usize));
    }

    pub(crate) fn get_archetype_graph_node_mut(&mut self, index: usize) -> Option<&mut ArchetypeGraphNode> {
        self.graph.get_mut(index)
    }

    pub fn get_entity_archetype_id(&self, entity: &EntityId) -> ArchetypeId {
        *self.entities.get(entity)
            .expect("the entity must initially be at least in an empty archetype")
    }

    pub fn get_archetype(&self, id: ArchetypeId) -> &Archetype {
        self.archetypes.get(id.as_usize())
            .expect("invalid archetype id")
    }

    pub fn get_archetype_mut(&mut self, id: ArchetypeId) -> &mut Archetype {
        self.archetypes.get_mut(id.as_usize())
            .expect("invalid archetype id")
    }
}

impl Index<ArchetypeId> for Archetypes {
    type Output = Archetype;

    #[inline]
    fn index(&self, index: ArchetypeId) -> &Self::Output {
        self.get_archetype(index)
    }
}

impl IndexMut<ArchetypeId> for Archetypes {
    #[inline]
    fn index_mut(&mut self, index: ArchetypeId) -> &mut Self::Output {
        self.get_archetype_mut(index)
    }
}
