use std::{any::TypeId, collections::HashSet, marker::PhantomData};

use uengine_cell::UnsafePtrCell;

use crate::{component::{Component, StoragePolicy}, entity::{self, EntityId}, storage::{Table, TableId}, world::{WorldCell, World}};

use super::{param::{ComponentSetRequestProxy, Fetch}, QueryParam};

pub struct With<C: Component>(PhantomData<C>);

pub struct Without<C: Component>(PhantomData<C>);

pub trait QueryFilter: QueryParam {
    fn filter<'w>(fetch: &mut Self::Fetch<'w>, entity: &EntityId) -> bool;
}

impl<C: Component> QueryParam for With<C> {
    type State = TypeId;
    type Fetch<'w> = ();
    type Item<'w> = ();

    const IS_DENSE: bool = {
        match C::STORAGE_POLICY {
            StoragePolicy::Dense => true,
            StoragePolicy::Sparse => false,
        }
    };

    fn init_state(_world: &mut World) -> Self::State {
        TypeId::of::<C>()
    }

    fn init_fetch<'w>(_world: WorldCell<'w>, _state: &Self::State) -> Self::Fetch<'w> { }

    fn include(mut proxy: ComponentSetRequestProxy) {
        proxy.include::<C>();
    }

    fn exclude(_: ComponentSetRequestProxy) { }

    fn set_table<'w>(_fetch: &mut Self::Fetch<'w>, _state: &Self::State, _table: uengine_cell::UnsafePtrCell<'w, Table>) { }

    fn fetch<'w>(_fetch: &mut Self::Fetch<'w>, _entity: &EntityId) -> Self::Item<'w> { }

    fn matches_component_set(state: &Self::State, set_contains_id: &impl Fn(TypeId) -> bool) -> bool {
        set_contains_id(*state)
    }
}

impl<C: Component> QueryFilter for With<C> {
    fn filter<'w>(_fetch: &mut Self::Fetch<'w>, _entity: &EntityId) -> bool {
        true
    }
}

impl<C: Component> QueryParam for Without<C> {
    type State = TypeId;
    type Fetch<'w> = Fetch<'w>;
    type Item<'w> = ();

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
            column: (C::STORAGE_POLICY == StoragePolicy::Sparse).then_some(
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
            )
        }
    }

    fn include(_proxy: ComponentSetRequestProxy) { }

    fn exclude(mut proxy: ComponentSetRequestProxy) {
        proxy.exclude::<C>();
    }

    fn set_table<'w>(_fetch: &mut Self::Fetch<'w>, _state: &Self::State, _table: UnsafePtrCell<'w, Table>) { }

    fn fetch<'w>(_fetch: &mut Self::Fetch<'w>, _entity: &EntityId) -> Self::Item<'w> { }

    fn matches_component_set(state: &Self::State, set_contains_id: &impl Fn(TypeId) -> bool) -> bool {
        !set_contains_id(*state)
    }
}

impl<C: Component> QueryFilter for Without<C> {
    fn filter<'w>(fetch: &mut Self::Fetch<'w>, entity: &EntityId) -> bool {
        match C::STORAGE_POLICY {
            StoragePolicy::Dense => true,
            StoragePolicy::Sparse => {
                let column = unsafe { fetch.column.unwrap().get() };
                column.entities().contains(entity)
            },
        }
    }
}

macro_rules! impl_filter {
    ($(($name:ident, $fetch:ident)),*) => {
        impl<$($name: QueryFilter + 'static),*> QueryFilter for ($($name,)*) {
            fn filter<'w>(_fetch: &mut Self::Fetch<'w>, _entity: &Entity) -> bool {
                let ($($fetch,)*) = _fetch;
                true $(&& $name::filter($fetch, _entity))*
            }
        }
    };
}

uengine_utils::all_tuples!(impl_filter, 0, 16, F, fetch);
