use std::{any::TypeId, collections::HashSet, hash::Hash};

use super::Archetype;

pub type ArchetypeId = usize;

type ArchetypeSearchNodeId = usize;

struct ArchetypeSearchNode {
    value: TypeId,
    parent: Option<ArchetypeSearchNodeId>,
    left: Option<ArchetypeSearchNodeId>,
    right: Option<ArchetypeSearchNodeId>
}

pub struct Archetypes {
    archetypes: Vec<Archetype>,
    search_tree: Vec<ArchetypeSearchNode>,
}

impl Archetypes {
    pub fn new() -> Self {
        let mut archetypes = Self {
            archetypes: Vec::new(),
            search_tree: Vec::new(),
        };

        archetypes.search_tree.push(ArchetypeSearchNode {
            value: TypeId::of::<()>(),
            parent: None,
            left: None,
            right: None
        });

        archetypes
    }

    pub fn get_or_create(&mut self, types_ids: HashSet<TypeId>) {

    }
}
