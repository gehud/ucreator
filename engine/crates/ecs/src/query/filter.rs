use std::{any::TypeId, marker::PhantomData};

use crate::{Entity, Table};

pub struct With<T>(PhantomData<T>);

pub struct Without<T>(PhantomData<T>);

pub trait QueryFilter {
    fn matches(table: &Table, entity: &Entity) -> bool;
}

impl<T: 'static> QueryFilter for With<T> {
    fn matches(table: &Table, entity: &Entity) -> bool {
        match table.get(&TypeId::of::<T>()) {
            None => false,
            Some(storage) => storage.contains(entity)
        }
    }
}

impl<T: 'static> QueryFilter for Without<T> {
    fn matches(table: &Table, entity: &Entity) -> bool {
        match table.get(&TypeId::of::<T>()) {
            None => true,
            Some(storage) => !storage.contains(entity)
        }
    }
}

macro_rules! impl_filter {
    () => {
        impl QueryFilter for () {
            fn matches(_: &Table, _: &Entity) -> bool {
                true
            }
        }
    };
    ($head:ident, $($tail:ident),*) => {
        impl<$head: QueryFilter, $($tail: QueryFilter),*> QueryFilter for ($head, $($tail),*) {
            fn matches(table: &Table, entity: &Entity) -> bool {
                $head::matches(table, entity) $(&& $tail::matches(table, entity))*
            }
        }
    };
}

uengine_utils::for_each_tuple_16!(impl_filter);
