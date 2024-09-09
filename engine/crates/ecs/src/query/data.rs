use crate::component::Component;

use super::QueryParam;

pub trait ReadOnlyQueryData: QueryData<ReadOnly = Self> { }

impl<T: Component> ReadOnlyQueryData for &T { }

pub trait QueryData: QueryParam {
    type ReadOnly: ReadOnlyQueryData<State = Self::State>;
}

impl<T: Component> QueryData for &T {
    type ReadOnly = Self;
}

impl<'a, T: Component> QueryData for &'a mut T {
    type ReadOnly = &'a T;
}

macro_rules! impl_query_data {
    () => {};
    ($head:ident, $($tail:ident),*) => {
        impl<$head: QueryData + 'static, $($tail: QueryData + 'static),*> QueryData for ($head, $($tail),*) {
            type ReadOnly = ($head::ReadOnly, $($tail::ReadOnly),*);
        }

        impl<$head: ReadOnlyQueryData + 'static, $($tail: ReadOnlyQueryData + 'static),*> ReadOnlyQueryData for ($head, $($tail),*) { }
    };
}

uengine_utils::for_each_tuple_16!(impl_query_data);
