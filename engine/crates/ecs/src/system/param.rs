use std::cell::{RefCell, UnsafeCell};

use crate::{UnsafeWorldPtrCell, World};

pub trait SystemParam {
    type State: 'static;
    type Item<'world, 'state>: SystemParam<State = Self::State>;

    fn init(world: &mut World) -> Self::State;

    fn get<'world, 'state>(world: UnsafeWorldPtrCell<'world>, state: &'state mut Self::State) -> Self::Item<'world, 'state>;
}

macro_rules! impl_param_set {
    () => {
        impl SystemParam for () {
            type State = ();
            type Item<'world, 'state> = Self;

            fn init(_: &mut World) -> Self::State {}

            fn get<'world, 'state>(_: UnsafeWorldPtrCell<'world>, _: &'state mut Self::State) -> Self::Item<'world, 'state> {}
        }
    };
    ($head:ident, $($tail:ident),*) => {
        #[allow(non_snake_case)]
        impl<$head: SystemParam, $($tail: SystemParam),*> SystemParam for ($head, $($tail),*) {
            type State = ($head::State, $($tail::State),*);
            type Item<'world, 'state> = ($head::Item<'world, 'state>, $($tail::Item<'world, 'state>),*);

            fn init(world: &mut World) -> Self::State {
                ($head::init(world), $($tail::init(world)),*)
            }

            fn get<'world, 'state>(world: UnsafeWorldPtrCell<'world>, state: &'state mut Self::State) -> Self::Item<'world, 'state> {
                let ($head, $($tail),*) = state;
                ($head::get(world, $head), $($tail::get(world, $tail)),*)
            }
        }
    };
}

uengine_utils::for_each_tuple_16!(impl_param_set);
