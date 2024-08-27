use crate::World;

pub trait SystemParam {
    type State: 'static;

    fn init(world: &mut World) -> Self::State;

    fn get(state: &mut Self::State) -> Self;
}

macro_rules! impl_param_set {
    () => {
        impl SystemParam for () {
            type State = ();

            fn init(_: &mut World) -> Self::State {}

            fn get(_: &mut Self::State) -> Self {}
        }
    };
    ($head:ident, $($tail:ident),*) => {
        #[allow(non_snake_case)]
        impl<$head: SystemParam, $($tail: SystemParam),*> SystemParam for ($head, $($tail),*) {
            type State = ($head::State, $($tail::State),*);

            fn init(world: &mut World) -> Self::State {
                ($head::init(world), $($tail::init(world)),*)
            }

            fn get(state: &mut Self::State) -> Self {
                let ($head, $($tail),*) = state;
                ($head::get($head), $($tail::get($tail)),*)
            }
        }
    };
}

uengine_utils::for_each_tuple_16!(impl_param_set);
