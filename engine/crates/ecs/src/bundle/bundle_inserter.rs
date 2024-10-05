use std::{any::TypeId, cell::Cell};

use crate::{archetype::{ArchetypeId, Archetypes}, component::{ComponentId, Components, StoragePolicy}, entity::{EntityId, EntityLocation}, storage::{Columns, Storages, Table, TableId, TableRow}, world::World};

use super::Bundle;

enum BundleInsertionResult {
    SameArchetype,
    NewArchetypeSameTable {
        new_archetype_id: ArchetypeId
    },
    NewArchetypeNewTable {
        new_archetype_id: ArchetypeId,
        new_table_id: TableId
    }
}

pub(crate) struct BundleInserter {
    archetype_id: ArchetypeId,
    table_id: TableId,
    result: BundleInsertionResult
}

impl BundleInserter {
    pub fn new<B: Bundle>(
        world: &mut World,
        archetype_id: ArchetypeId
    ) -> Self {
        let new_archetype_id = Self::add_bundle_to_archetype::<B>(
            world,
            archetype_id
        );

        if new_archetype_id == archetype_id {
            let archetype = &world.archetypes()[archetype_id];

            Self {
                archetype_id,
                table_id: archetype.table_id(),
                result: BundleInsertionResult::SameArchetype
            }
        } else {
            let archetype = &world.archetypes()[archetype_id];
            let new_archetype = &world.archetypes()[new_archetype_id];

            let table_id = archetype.table_id();
            let new_table_id = new_archetype.table_id();

            if table_id == new_table_id {
                Self {
                    archetype_id,
                    table_id,
                    result: BundleInsertionResult::NewArchetypeSameTable {
                        new_archetype_id: new_archetype.id()
                    }
                }
            } else {
                Self {
                    archetype_id,
                    table_id,
                    result: BundleInsertionResult::NewArchetypeNewTable {
                        new_archetype_id: new_archetype.id(),
                        new_table_id
                    }
                }
            }
        }
    }

    fn add_bundle_to_archetype<B: Bundle>(
        world: &mut World,
        archetype_id: ArchetypeId
    ) -> ArchetypeId {
        let mut new_dense_component_ids = Vec::new();
        let mut new_sparse_component_ids = Vec::new();
        let mut components_ids = Vec::new();

        B::get_component_ids(world.components(), &mut |components_id| {
            components_ids.push(components_id);
        });

        let archetype = &world.archetypes()[archetype_id];

        for component_id in &components_ids {
            if !archetype.component_ids().contains(&component_id) {
                match world.components().get_component_info(*component_id).storage_policy() {
                    StoragePolicy::Dense => {
                        new_dense_component_ids.push(*component_id);
                    },
                    StoragePolicy::Sparse => {
                        new_sparse_component_ids.push(*component_id);
                    },
                }
            }
        }

        if new_dense_component_ids.is_empty() && new_sparse_component_ids.is_empty() {
            archetype_id
        } else {
            let table_id = if new_dense_component_ids.is_empty() {
                world.archetypes().get_archetype(archetype_id).table_id()
            } else {
                let archetype = world.archetypes().get_archetype(archetype_id);
                let table = world.storages().tables().get_table(archetype.table_id()).unwrap();
                new_dense_component_ids.extend(table.components());
                new_dense_component_ids.sort();

                world.storages_mut().tables_mut().get_or_insert_table(&new_dense_component_ids)
            };

            let column_ids = if new_sparse_component_ids.is_empty() {
                world.archetypes().get_archetype(archetype_id)
                    .column_ids().iter().cloned().collect()
            } else {
                let archetype = world.archetypes().get_archetype(archetype_id);
                new_sparse_component_ids.extend(archetype.column_ids());
                new_sparse_component_ids.sort();
                new_sparse_component_ids
            };

            world.archetypes_mut().get_or_insert_archetype(
                table_id,
                column_ids,
                components_ids
            )
        }
    }

    fn write_components<B: Bundle>(
        &mut self,
        world: &mut World,
        entity_id: EntityId,
        table_row: TableRow,
        bundle: B
    ) {
        let mut component_index = 0usize;
        bundle.get_component_data(&mut |storage_type, data| {
            let component_ids = world.archetypes()
                .get_archetype(self.archetype_id)
                .component_ids();

            let component_id = component_ids[component_index];

            match storage_type {
                StoragePolicy::Dense => {
                    let table = world.storages_mut().tables_mut().get_table_mut(self.table_id);
                    let column = table.get_column_mut(component_id).unwrap();
                    column.write_component(table_row, data);
                },
                StoragePolicy::Sparse => {
                    let columns = world.storages_mut().columns_mut();
                    let column = columns.get_column_mut(component_id).unwrap();
                    column.insert_data(entity_id, data);
                },
            };

            component_index += 1;
        });
    }

    pub fn insert<B: Bundle>(
        &mut self,
        world: &mut World,
        entity_id: EntityId,
        location: EntityLocation,
        bundle: B
    ) -> EntityLocation {
        match self.result {
            BundleInsertionResult::SameArchetype => {
                self.write_components(world, entity_id, location.table_row(), bundle);
                location
            },
            BundleInsertionResult::NewArchetypeSameTable { new_archetype_id } => {
                let new_location = EntityLocation::new(new_archetype_id, location.table_row());
                self.archetype_id = new_archetype_id;
                *world.entities_mut().get_entity_location_mut(&entity_id) = new_location;
                self.write_components(world, entity_id, location.table_row(), bundle);
                new_location
            },
            BundleInsertionResult::NewArchetypeNewTable { new_archetype_id, new_table_id } => {
                let old_table = world.storages_mut().tables_mut().get_table_mut(self.table_id);

                let new_table = world.storages_mut().tables_mut().get_table_mut(new_table_id);
                old_table.move_entity(entity_id, new_table);
                location
            },
        }
    }
}
