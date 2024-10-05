use std::{any::Any, collections::HashSet, fmt::Debug, ptr};

use uengine_cell::UnsafePtrCell;

use crate::{component::StoragePolicy, storage::{Column, ColumnId, Table, TableId}, system::SystemParam, world::{WorldCell, World}};

use super::{param::ComponentSetRequest, QueryData, QueryFilter};

#[derive(Debug)]
struct Archetype {
    dense_id: TableId,
    sparse_ids: Vec<ColumnId>,
}

pub struct QueryState<D: QueryData, F: QueryFilter> {
    data: D::State,
    filter: F::State,
    matched_archetypes: Vec<Archetype>
}

impl<D: QueryData, F: QueryFilter> QueryState<D, F> {
    pub fn as_readonly(&self) -> &QueryState<D::ReadOnly, F> {
        self.as_transmuted()
    }

    fn as_transmuted<
        DT: QueryData<State = D::State>,
        FT: QueryFilter<State = F::State>,
    >(
        &self,
    ) -> &QueryState<DT, FT> {
        unsafe {
            &*ptr::from_ref(self).cast::<QueryState<DT, FT>>()
        }
    }
}

pub struct Query<'w, 's, D: QueryData, F: QueryFilter = ()> {
    world: WorldCell<'w>,
    state: &'s QueryState<D, F>
}

impl<'w, 's, D: QueryData, F: QueryFilter> Query<'w, 's, D, F> {
    fn new(world: WorldCell<'w>, state: &'s QueryState<D, F>) -> Self {
        Self {
            world,
            state
        }
    }

    pub fn iter(&self) -> QueryIterator<'w, 's, D::ReadOnly, F> {
        QueryIterator::new(self.world, self.state.as_readonly())
    }

    pub fn iter_mut(&mut self) -> QueryIterator<'w, 's, D, F> {
        QueryIterator::new(self.world, self.state)
    }

    fn get_matched_archetypes(world: &mut World) -> Vec<Archetype> {
        let mut archetypes = Vec::<Archetype>::new();

        let mut request = ComponentSetRequest::new();
        D::include(request.proxy());
        D::exclude(request.proxy());
        F::include(request.proxy());
        F::exclude(request.proxy());

        for component_set in request.component_sets() {
            let mut dense_components = Vec::new();
            let mut sparse_components = Vec::new();

            for component_info in component_set {
                let components = match component_info.storage_policy() {
                    StoragePolicy::Dense => &mut dense_components,
                    StoragePolicy::Sparse => &mut sparse_components,
                };

                components.push(component_info);
            }

            let mut table_type_ids = Vec::new();
            for component_info in dense_components {
                table_type_ids.push(component_info.type_id());
            }

            let (table_id, _) = world.storages.dense.get_or_insert_table(&table_type_ids);

            let mut column_ids = Vec::new();
            for component_info in sparse_components {
                world.storages.sparse.get_or_create_column(*component_info.type_info());
                column_ids.push(*component_info.type_info().id());
            }

            let archetype = Archetype {
                dense_id: table_id,
                sparse_ids: column_ids
            };

            archetypes.push(archetype);
        }

        archetypes
    }
}

impl<D: QueryData + 'static, F: QueryFilter + 'static> SystemParam for Query<'_, '_, D, F> {
    type State = QueryState<D, F>;
    type Item<'w, 's> = Query<'w, 's, D, F>;

    fn init(world: &mut World) -> Self::State {
        Self::State {
            data: D::init_state(world),
            filter: F::init_state(world),
            matched_archetypes: Self::get_matched_archetypes(world)
        }
    }

    fn get<'w, 's>(world: WorldCell<'w>, state: &'s mut Self::State) -> Self::Item<'w, 's> {
        Query::new(world, state)
    }
}

pub struct QueryIterator<'w, 's, D: QueryData, F: QueryFilter> {
    world: WorldCell<'w>,
    state: &'s QueryState<D, F>,
    data: D::Fetch<'w>,
    filter: F::Fetch<'w>,
    pos: usize,
    archetype_index: usize
}

impl<'w, 's, D: QueryData, F: QueryFilter> QueryIterator<'w, 's, D, F> {
    pub fn new(world: WorldCell<'w>, state: &'s QueryState<D, F>) -> Self {
        Self {
            world,
            state,
            data: D::init_fetch(world, &state.data),
            filter: F::init_fetch(world, &state.filter),
            pos: 0usize,
            archetype_index: 0usize
        }
    }
}

impl<'w, 's, D: QueryData, F: QueryFilter> Iterator for QueryIterator<'w, 's, D, F> {
    type Item = D::Item<'w>;

    fn next(&mut self) -> Option<Self::Item> {
        if D::IS_DENSE {
            loop {
                let archetype = self.state.matched_archetypes.get(self.archetype_index).unwrap();
                let table = unsafe { self.world.get().storages.dense.get_table(archetype.dense_id).unwrap() };
                let len = table.len();
                if self.pos == len {
                    if self.archetype_index == self.state.matched_archetypes.len() - 1 {
                        return None;
                    }

                    let table_ptr = UnsafePtrCell::from_mut(unsafe { self.world.get_mut().storages.dense.get_table_mut(archetype.dense_id).unwrap() });
                    D::set_table(&mut self.data, &self.state.data, table_ptr);
                    F::set_table(&mut self.filter, &self.state.filter, table_ptr);

                    self.archetype_index += 1;
                    continue;
                }

                let entity = table.entities().get(self.pos).unwrap();

                if !F::filter(&mut self.filter, entity) {
                    self.pos += 1;
                    continue;
                }

                self.pos += 1;
                return Some(D::fetch(&mut self.data, entity));
            }
        } else {
            None
        }
    }
}
