use crate::component::Component;

use super::QueryParam;

pub trait ReadOnlyQueryData: QueryData<ReadOnly = Self> { }

impl<C: Component> ReadOnlyQueryData for &C { }

impl<D: ReadOnlyQueryData> ReadOnlyQueryData for Option<D> { }

pub trait QueryData: QueryParam {
    type ReadOnly: ReadOnlyQueryData<State = Self::State>;
}

impl<C: Component> QueryData for &C {
    type ReadOnly = Self;
}

impl<'a, C: Component> QueryData for &'a mut C {
    type ReadOnly = &'a C;
}

impl<D: QueryData> QueryData for Option<D> {
    type ReadOnly = Option<D::ReadOnly>;
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
