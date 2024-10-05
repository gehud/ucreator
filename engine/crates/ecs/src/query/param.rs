use std::{any::{Any, TypeId}, collections::HashSet, fmt::Write, hash::Hash, mem::take};

use uengine_cell::UnsafePtrCell;

use crate::{component::{Component, ComponentInfo, StoragePolicy}, entity::{self, EntityId}, storage::{Column, Table, TableId}, world::{WorldCell, World}};

pub struct ComponentSetRequest {
    sets: Vec<HashSet<ComponentInfo>>
}

pub struct ComponentSetRequestProxy<'a> {
    request: &'a mut ComponentSetRequest,
    cursor: usize,
}

impl<'a> ComponentSetRequestProxy<'a> {
    pub fn new(request: &'a mut ComponentSetRequest) -> Self {
        Self {
            request,
            cursor: 0usize
        }
    }

    pub fn clone<'b>(&'b mut self) -> ComponentSetRequestProxy<'b> {
        ComponentSetRequestProxy {
            request: &mut self.request,
            cursor: self.cursor
        }
    }

    pub fn next(&mut self) {
        self.cursor += 1;
    }

    pub fn include<C: Component>(&mut self) {
        let set = match self.request.sets.get_mut(self.cursor) {
            Some(set) => set,
            None => {
                let set = match self.request.sets.first() {
                    Some(set) => set.clone(),
                    None => HashSet::new(),
                };

                self.request.sets.push(set);
                self.request.sets.last_mut().unwrap()
            },
        };

        set.insert(ComponentInfo::of::<C>());

        self.cursor += 1;
    }

    pub fn exclude<C: Component>(&mut self) {
        let component_info = &ComponentInfo::of::<C>();

        for set in &mut self.request.sets {
            set.remove(component_info);
        }
    }
}

impl ComponentSetRequest {
    pub fn new() -> Self {
        Self {
            sets: Vec::new()
        }
    }

    pub fn proxy<'a>(&'a mut self) -> ComponentSetRequestProxy<'a> {
        ComponentSetRequestProxy::new(self)
    }

    pub fn component_sets(&self) -> &Vec<HashSet<ComponentInfo>> {
        &self.sets
    }
}

pub trait QueryParam {
    type State;
    type Fetch<'w>;
    type Item<'w>;

    const IS_DENSE: bool;

    fn init_state(world: &mut World) -> Self::State;

    fn init_fetch<'w>(world: WorldCell<'w>, state: &Self::State) -> Self::Fetch<'w>;

    fn include(proxy: ComponentSetRequestProxy);

    fn exclude(proxy: ComponentSetRequestProxy);

    fn set_table<'w>(fetch: &mut Self::Fetch<'w>, state: &Self::State, table: UnsafePtrCell<'w, Table>);

    fn fetch<'w>(fetch: &mut Self::Fetch<'w>, entity: &EntityId) -> Self::Item<'w>;

    fn matches_component_set(state: &Self::State, set_contains_id: &impl Fn(TypeId) -> bool) -> bool;
}

pub struct Fetch<'a> {
    pub(crate) table: Option<UnsafePtrCell<'a, Table>>,
    pub(crate) column: Option<UnsafePtrCell<'a, Column>>
}

impl<C: Component> QueryParam for &C {
    type State = TypeId;
    type Fetch<'w> = Fetch<'w>;
    type Item<'w> = &'w C;

    const IS_DENSE: bool = {
        match C::STORAGE_POLICY {
            StoragePolicy::Dense => true,
            StoragePolicy::Sparse => false,
        }
    };

    fn init_state(_world: &mut World) -> Self::State {
        TypeId::of::<C>()
    }

    fn init_fetch<'w>(world: WorldCell<'w>, state: &Self::State) -> Self::Fetch<'w> {
        Self::Fetch {
            table: None,
            column: (C::STORAGE_POLICY == StoragePolicy::Sparse).then(|| {
                UnsafePtrCell::from_ref(
                    unsafe {
                        world
                            .get()
                            .storages
                            .sparse
                            .get_column(state)
                            .unwrap()
                    }
                )
            })
        }
    }

    fn include(mut proxy: ComponentSetRequestProxy) {
        proxy.include::<C>();
    }

    fn exclude(_proxy: ComponentSetRequestProxy) { }

    fn set_table<'w>(fetch: &mut Self::Fetch<'w>, _state: &Self::State, table: UnsafePtrCell<'w, Table>) {
        fetch.table = Some(table);
    }

    fn fetch<'w>(fetch: &mut Self::Fetch<'w>, entity: &EntityId) -> Self::Item<'w> {
        match C::STORAGE_POLICY {
            StoragePolicy::Dense => {
                let table = fetch.table.unwrap();
                unsafe { table.get() }.get::<C>(entity).unwrap()
            },
            StoragePolicy::Sparse => {
                let column = fetch.column.unwrap();
                unsafe { column.get() }.get::<C>(entity).unwrap()
            },
        }
    }

    fn matches_component_set(state: &Self::State, set_contains_id: &impl Fn(TypeId) -> bool) -> bool {
        set_contains_id(*state)
    }
}

impl<'a, C: Component> QueryParam for &'a mut C {
    type State = TypeId;
    type Fetch<'w> = Fetch<'w>;
    type Item<'w> = &'w mut C;

    const IS_DENSE: bool = {
        match C::STORAGE_POLICY {
            StoragePolicy::Dense => true,
            StoragePolicy::Sparse => false,
        }
    };

    fn init_state(_world: &mut World) -> Self::State {
        TypeId::of::<C>()
    }

    fn init_fetch<'w>(world: WorldCell<'w>, state: &Self::State) -> Self::Fetch<'w> {
        Self::Fetch {
            table: None,
            column: (C::STORAGE_POLICY == StoragePolicy::Sparse).then(|| {
                UnsafePtrCell::from_mut(
                    unsafe {
                        world
                            .get_mut()
                            .storages
                            .sparse
                            .get_column_mut(state)
                            .unwrap()
                    }
                )
            })
        }
    }

    fn include(mut proxy: ComponentSetRequestProxy) {
        proxy.include::<C>();
    }

    fn exclude(_proxy: ComponentSetRequestProxy) { }

    fn set_table<'w>(fetch: &mut Self::Fetch<'w>, _state: &Self::State, table: UnsafePtrCell<'w, Table>) {
        fetch.table = Some(table);
    }

    fn fetch<'w>(fetch: &mut Self::Fetch<'w>, entity: &EntityId) -> Self::Item<'w> {
        match C::STORAGE_POLICY {
            StoragePolicy::Dense => {
                let table = fetch.table.unwrap();
                unsafe { table.get_mut() }.get_mut::<C>(entity).unwrap()
            },
            StoragePolicy::Sparse => {
                let column = fetch.column.unwrap();
                unsafe { column.get_mut() }.get_mut::<C>(entity).unwrap()
            },
        }
    }

    fn matches_component_set(state: &Self::State, set_contains_id: &impl Fn(TypeId) -> bool) -> bool {
        set_contains_id(*state)
    }
}

pub struct OptionFetch<'a, P: QueryParam> {
    fetch: P::Fetch<'a>,
    matches: bool
}

impl<P: QueryParam> QueryParam for Option<P> {
    type State = P::State;
    type Fetch<'w> = OptionFetch<'w, P>;
    type Item<'w> = Option<P::Item<'w>>;

    const IS_DENSE: bool = P::IS_DENSE;

    fn init_state(world: &mut World) -> Self::State {
        P::init_state(world)
    }

    fn init_fetch<'w>(world: WorldCell<'w>, state: &Self::State) -> Self::Fetch<'w> {
        OptionFetch {
            fetch: P::init_fetch(world, state),
            matches: false
        }
    }

    fn include(mut proxy: ComponentSetRequestProxy) {
        proxy.next();
        P::include(proxy);
    }

    fn exclude(proxy: ComponentSetRequestProxy) {
        P::exclude(proxy)
    }

    fn set_table<'w>(fetch: &mut Self::Fetch<'w>, state: &Self::State, table: UnsafePtrCell<'w, Table>) {
        fetch.matches = P::matches_component_set(state, &|id| unsafe { table.get() }.has_column(&id));
        if fetch.matches {
            P::set_table(&mut fetch.fetch, state, table);
        }
    }

    fn fetch<'w>(fetch: &mut Self::Fetch<'w>, entity: &EntityId) -> Self::Item<'w> {
        fetch
            .matches
            .then_some(P::fetch(&mut fetch.fetch, entity))
    }

    fn matches_component_set(_: &Self::State, _: &impl Fn(TypeId) -> bool) -> bool {
        true
    }
}

macro_rules! impl_query_param {
    ($(($name:ident, $state:ident, $fetch:ident)),*) => {
        impl<$($name: QueryParam + 'static),*> QueryParam for ($($name,)*) {
            type State = ($($name::State,)*);
            type Fetch<'a> = ($($name::Fetch<'a>,)*);
            type Item<'a> = ($($name::Item<'a>,)*);

            const IS_DENSE: bool = true $(&& $name::IS_DENSE)*;

            fn init_state(_world: &mut World) -> Self::State {
                ($($name::init_state(_world),)*)
            }

            fn init_fetch<'w>(_world: UnsafeWorldPtrCell<'w>, _state: &Self::State) -> Self::Fetch<'w> {
                let ($($state,)*) = _state;
                ($($name::init_fetch(_world, $state),)*)
            }

            fn include(mut _proxy: ComponentSetRequestProxy) {
                $($name::include(_proxy.clone());)*
            }

            fn exclude(mut _proxy: ComponentSetRequestProxy) {
                $($name::exclude(_proxy.clone());)*
            }

            fn set_table<'w>(_fetch: &mut Self::Fetch<'w>, _state: &Self::State, _table: UnsafePtrCell<'w, Table>) {
                let ($($fetch,)*) = _fetch;
                let ($($state,)*) = _state;
                $($name::set_table($fetch, $state, _table);)*
            }

            fn fetch<'w>(_fetch: &mut Self::Fetch<'w>, _entity: &Entity) -> Self::Item<'w> {
                let ($($fetch,)*) = _fetch;
                ($($name::fetch($fetch, _entity),)*)
            }

            fn matches_component_set(_state: &Self::State, _set_contains_id: &impl Fn(TypeId) -> bool) -> bool {
                let ($($state,)*) = _state;
                true $(&& $name::matches_component_set($state, _set_contains_id))*
            }
        }
    };
}

uengine_utils::all_tuples!(impl_query_param, 0, 16, P, state, fetch);
