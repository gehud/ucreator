use std::{any::TypeId, cell::UnsafeCell};

use crate::{Entity, Storage, Table};

pub trait QueryData {
    type Type<'a>;

    fn contains(components: &Table) -> bool;

    fn primary<'a>(components: &'a Table) -> &'a Storage;

    fn matches(components: &Table, entity: &Entity) -> bool;

    fn fetch<'a>(components: &'a UnsafeCell<Table>, entity: &Entity) -> Self::Type<'a>;
}

impl<T: 'static> QueryData for &T {
    type Type<'a> = &'a T;

    fn contains(components: &Table) -> bool {
        components.contains_key(&TypeId::of::<T>())
    }

    fn fetch<'a>(components: &'a UnsafeCell<Table>, entity: &Entity) -> <&'a T as QueryData>::Type<'a> {
        let storage = unsafe { &mut *components.get() };
        storage.get_mut(&TypeId::of::<T>()).unwrap().get_mut(entity).unwrap()
    }

    fn primary<'a>(components: &'a Table) -> &'a Storage {
        components.get(&TypeId::of::<T>()).unwrap()
    }

    fn matches(components: &Table, entity: &Entity) -> bool {
        components.get(&TypeId::of::<T>()).unwrap().contains(entity)
    }
}

impl<T: 'static> QueryData for &mut T {
    type Type<'a> = &'a mut T;

    fn contains(components: &Table) -> bool {
        components.contains_key(&TypeId::of::<T>())
    }

    fn fetch<'a>(components: &'a UnsafeCell<Table>, entity: &Entity) -> <&'a mut T as QueryData>::Type<'a> {
        let storage = unsafe { &mut *components.get() };
        storage.get_mut(&TypeId::of::<T>()).unwrap().get_mut(entity).unwrap()
    }

    fn primary<'a>(components: &'a Table) -> &'a Storage {
        components.get(&TypeId::of::<T>()).unwrap()
    }

    fn matches(components: &Table, entity: &Entity) -> bool {
        components.get(&TypeId::of::<T>()).unwrap().contains(entity)
    }
}

impl QueryData for Entity {
    type Type<'a> = Entity;

    fn contains(_: &Table) -> bool {
        true
    }

    fn primary<'a>(components: &'a Table) -> &'a Storage {
        components.get(&TypeId::of::<()>()).unwrap()
    }

    fn matches(_: &Table, _: &Entity) -> bool {
        true
    }

    fn fetch<'a>(_: &'a UnsafeCell<Table>, entity: &Entity) -> Self::Type<'a> {
        *entity
    }
}

macro_rules! impl_data {
    () => {};
    ($head:ident, $($tail:ident),*) => {
        impl<$head: QueryData, $($tail: QueryData),*> QueryData for ($head, $($tail),*)
        {
            type Type<'a> = (<$head as QueryData>::Type<'a>, $(<$tail as QueryData>::Type<'a>),*);

            fn contains(components: &Table) -> bool {
                $head::contains(components) $(&& $tail::contains(components))*
            }

            fn fetch<'a>(components: &'a UnsafeCell<Table>, entity: &Entity) -> <($head, $($tail),*) as QueryData>::Type<'a> {
                ($head::fetch(components, entity), $($tail::fetch(components, entity)),*)
            }

            fn primary<'a>(components: &'a Table) -> &'a Storage {
                $head::primary(components)
            }

            fn matches(components: &Table, entity: &Entity) -> bool {
                $head::matches(components, entity) $(&& $tail::matches(components, entity))*
            }
        }
    };
}

uengine_utils::for_each_tuple_16!(impl_data);
