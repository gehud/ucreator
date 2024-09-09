use std::{any::TypeId, collections::HashSet, marker::PhantomData};

use crate::{component::Component, World};

pub trait QueryParam {
    type Item<'a>;
    type State;

    fn include(types: &mut HashSet<TypeId>);

    fn exclude(types: &mut HashSet<TypeId>);

    fn init(world: &mut World) -> Self::State;
}

impl QueryParam for () {
    type Item<'a> = ();
    type State = ();

    fn include(_: &mut HashSet<TypeId>) { }

    fn exclude(_: &mut HashSet<TypeId>) { }

    fn init(_: &mut World) -> Self::State { }
}

impl<T: Component> QueryParam for &T {
    type Item<'a> = &'a T;
    type State = TypeId;

    fn include(types: &mut HashSet<TypeId>) {
        types.insert(TypeId::of::<T>());
    }

    fn exclude(_: &mut HashSet<TypeId>) { }

    fn init(_: &mut World) -> Self::State {
        TypeId::of::<T>()
    }
}

impl<'a, T: Component> QueryParam for &'a mut T {
    type Item<'w> = &'w mut T;
    type State = TypeId;

    fn include(types: &mut HashSet<TypeId>) {
        types.insert(TypeId::of::<T>());
    }

    fn exclude(_: &mut HashSet<TypeId>) { }

    fn init(_: &mut World) -> Self::State {
        TypeId::of::<T>()
    }
}

macro_rules! impl_query_param {
    () => {};
    ($head:ident, $($tail:ident),*) => {
        impl<$head: QueryParam + 'static, $($tail: QueryParam + 'static),*> QueryParam for ($head, $($tail),*) {
            type Item<'a> = ($head::Item<'a>, $($tail::Item<'a>),*);
            type State = ($head::State, $($tail::State),*);

            fn include(types: &mut HashSet<TypeId>) {
                $head::include(types) $(; $tail::include(types))*;
            }

            fn exclude(types: &mut HashSet<TypeId>) {
                $head::exclude(types) $(; $tail::exclude(types))*;
            }

            fn init(world: &mut World) -> Self::State {
                ($head::init(world), $($tail::init(world)),*)
            }
        }
    };
}

uengine_utils::for_each_tuple_16!(impl_query_param);
