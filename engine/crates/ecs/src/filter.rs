use std::{any::TypeId, marker::PhantomData};

use super::{Entity, Table};

pub struct With<T>(PhantomData<T>);

pub struct Without<T>(PhantomData<T>);

pub trait Filter {
    fn matches(table: &Table, entity: &Entity) -> bool;
}

impl<T: 'static> Filter for With<T> {
    fn matches(table: &Table, entity: &Entity) -> bool {
        match table.get(&TypeId::of::<T>()) {
            None => false,
            Some(storage) => storage.contains(entity)
        }
    }
}

impl<T: 'static> Filter for Without<T> {
    fn matches(table: &Table, entity: &Entity) -> bool {
        match table.get(&TypeId::of::<T>()) {
            None => true,
            Some(storage) => !storage.contains(entity)
        }
    }
}

macro_rules! impl_filter {
    () => {
        impl Filter for () {
            fn matches(_: &Table, _: &Entity) -> bool {
                true
            }
        }
    };
    ($head:ident, $($tail:ident),*) => {
        impl<$head: Filter, $($tail: Filter),*> Filter for ($head, $($tail),*) {
            fn matches(table: &Table, entity: &Entity) -> bool {
                $head::matches(table, entity) $(&& $tail::matches(table, entity))*
            }
        }
    };
}

uengine_utils::for_each_tuple_16!(impl_filter);
