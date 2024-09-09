use std::ptr;

use crate::{system::SystemParam, QueryData, QueryFilter, UnsafeWorldPtrCell, World};

pub struct QueryState<D: QueryData, F: QueryFilter> {
    data_state: D::State,
    filter_state: F::State
}

impl<D: QueryData, F: QueryFilter> QueryState<D, F> {
    pub fn as_readonly(&self) -> &QueryState<D::ReadOnly, F> {
        unsafe {
            self.as_transmuted_state()
        }
    }

    unsafe fn as_transmuted_state<
        DT: QueryData<State = D::State>,
        FT: QueryFilter<State = F::State>,
    >(
        &self,
    ) -> &QueryState<DT, FT> {
        &*ptr::from_ref(self).cast::<QueryState<DT, FT>>()
    }
}

pub struct Query<'world, 'state, D: QueryData, F: QueryFilter = ()> {
    world: UnsafeWorldPtrCell<'world>,
    state: &'state QueryState<D, F>
}

impl<'world, 'state, D: QueryData, F: QueryFilter> Query<'world, 'state, D, F> {
    fn new(world: UnsafeWorldPtrCell<'world>, state: &'state QueryState<D, F>) -> Self {
        Self {
            world,
            state
        }
    }

    pub fn iter(&self) -> QueryIterator<'world, 'state, D::ReadOnly, F> {
        QueryIterator::new(self.world, self.state.as_readonly())
    }

    pub fn iter_mut(&mut self) -> QueryIterator<'world, 'state, D, F> {
        QueryIterator::new(self.world, self.state)
    }
}

impl<D: QueryData + 'static, F: QueryFilter + 'static> SystemParam for Query<'_, '_, D, F> {
    type State = QueryState<D, F>;
    type Item<'world, 'state> = Query<'world, 'state, D, F>;

    fn init(world: &mut World) -> Self::State {
        Self::State {
            data_state: D::init(world),
            filter_state: F::init(world)
        }
    }

    fn get<'world, 'state>(world: UnsafeWorldPtrCell<'world>, state: &'state mut Self::State) -> Self::Item<'world, 'state> {
        Query::new(world, state)
    }
}

pub struct QueryIterator<'world, 'state, D: QueryData, F: QueryFilter> {
    world: UnsafeWorldPtrCell<'world>,
    state: &'state QueryState<D, F>,
    row: usize
}

impl<'world, 'state, D: QueryData, F: QueryFilter> QueryIterator<'world, 'state, D, F> {
    pub fn new(world: UnsafeWorldPtrCell<'world>, state: &'state QueryState<D, F>) -> Self {
        Self {
            world,
            state,
            row: 0usize
        }
    }
}

impl<'world, 'state, D: QueryData, F: QueryFilter> Iterator for QueryIterator<'world, 'state, D, F> {
    type Item = D::Item<'world>;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
