use std::{any::TypeId, collections::HashSet, marker::PhantomData};

use crate::{component::Component, World};

use super::QueryParam;

pub struct With<T: Component>(PhantomData<T>);

pub struct Without<T: Component>(PhantomData<T>);

pub trait QueryFilter: QueryParam { }

impl<T: Component> QueryParam for With<T> {
    type Item<'a> = ();

    type State = ();

    fn include(types: &mut HashSet<TypeId>) {
        types.insert(TypeId::of::<T>());
    }

    fn exclude(_: &mut HashSet<TypeId>) { }

    fn init(_: &mut World) -> Self::State { }
}

impl<T: Component> QueryFilter for With<T> { }

impl<T: Component> QueryParam for Without<T> {
    type Item<'a> = ();

    type State = ();

    fn include(_: &mut HashSet<TypeId>) { }

    fn exclude(types: &mut HashSet<TypeId>) {
        types.remove(&TypeId::of::<T>());
    }

    fn init(_: &mut World) -> Self::State { }
}

macro_rules! impl_filter {
    () => {
        impl QueryFilter for () { }
    };
    ($head:ident, $($tail:ident),*) => {
        impl<$head: QueryFilter + 'static, $($tail: QueryFilter + 'static),*> QueryFilter for ($head, $($tail),*) { }
    };
}

uengine_utils::for_each_tuple_16!(impl_filter);
