use std::cell::{Cell, RefCell, UnsafeCell};

use crate::world::World;

pub trait SystemParam {
    type State: 'static;
    type Item<'w, 's>: SystemParam<State = Self::State>;

    fn init(world: &mut World) -> Self::State;

    fn get<'w, 's>(world: &'w Cell<World>, state: &'s mut Self::State) -> Self::Item<'w, 's>;
}

macro_rules! impl_param_set {
    () => {
        impl SystemParam for () {
            type State = ();
            type Item<'w, 's> = Self;

            fn init(_: &mut World) -> Self::State {}

            fn get<'w, 's>(_: &'w Cell<World>, _: &'s mut Self::State) -> Self::Item<'w, 's> {}
        }
    };
    ($head:ident, $($tail:ident),*) => {
        #[allow(non_snake_case)]
        impl<$head: SystemParam, $($tail: SystemParam),*> SystemParam for ($head, $($tail),*) {
            type State = ($head::State, $($tail::State),*);
            type Item<'w, 's> = ($head::Item<'w, 's>, $($tail::Item<'w, 's>),*);

            fn init(world: &mut World) -> Self::State {
                ($head::init(world), $($tail::init(world)),*)
            }

            fn get<'w, 's>(world: &'w Cell<World>, state: &'s mut Self::State) -> Self::Item<'w, 's> {
                let ($head, $($tail),*) = state;
                ($head::get(world, $head), $($tail::get(world, $tail)),*)
            }
        }
    };
}

uengine_utils::for_each_tuple_16!(impl_param_set);
