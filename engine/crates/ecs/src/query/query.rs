use std::marker::PhantomData;

use crate::{system::SystemParam, QueryData, QueryFilter, World};

#[derive(Clone, Copy)]
pub struct QueryState {
    world: *mut World
}

pub struct Query<D: QueryData, F: QueryFilter = ()> {
    d: PhantomData<D>,
    f: PhantomData<F>,
    state: QueryState
}

impl<D: QueryData + 'static, F: QueryFilter + 'static> Query<D, F> {
    pub fn new(world: &mut World) -> Self {
        Self {
            d: PhantomData,
            f: PhantomData,
            state: QueryState {
                world
            }
        }
    }

    fn restore(state: QueryState) -> Self {
        Self {
            d: PhantomData,
            f: PhantomData,
            state
        }
    }
}

impl<D: QueryData + 'static, F: QueryFilter + 'static> SystemParam for Query<D, F> {
    type State = QueryState;

    fn init(world: &mut World) -> Self::State {
        Self::new(world).state
    }

    fn get(state: &mut Self::State) -> Self {
        Self::restore(*state)
    }
}
