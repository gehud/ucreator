use std::{any::TypeId, cell::UnsafeCell};

use super::{Entity, Storage, Table};

use crate::utils::for_each_tuple_16;

pub trait Data {
    type Type<'a>;

    fn contains(components: &Table) -> bool;

    fn primary<'a>(components: &'a Table) -> &'a Storage;

    fn matches(components: &Table, entity: &Entity) -> bool;

    fn fetch<'a>(components: &'a UnsafeCell<Table>, entity: &Entity) -> Self::Type<'a>;
}

impl<T: 'static> Data for &T {
    type Type<'a> = &'a T;

    fn contains(components: &Table) -> bool {
        components.contains_key(&TypeId::of::<T>())
    }

    fn fetch<'a>(components: &'a UnsafeCell<Table>, entity: &Entity) -> <&'a T as Data>::Type<'a> {
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

impl<T: 'static> Data for &mut T {
    type Type<'a> = &'a mut T;

    fn contains(components: &Table) -> bool {
        components.contains_key(&TypeId::of::<T>())
    }

    fn fetch<'a>(components: &'a UnsafeCell<Table>, entity: &Entity) -> <&'a mut T as Data>::Type<'a> {
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

macro_rules! impl_data {
    () => {};
    ($head:ident, $($tail:ident),*) => {
        impl<$head: Data, $($tail: Data),*> Data for ($head, $($tail),*)
        {
            type Type<'a> = (<$head as Data>::Type<'a>, $(<$tail as Data>::Type<'a>),*);

            fn contains(components: &Table) -> bool {
                $head::contains(components) $(&& $tail::contains(components))*
            }

            fn fetch<'a>(components: &'a UnsafeCell<Table>, entity: &Entity) -> <($head, $($tail),*) as Data>::Type<'a> {
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

for_each_tuple_16!(impl_data);
