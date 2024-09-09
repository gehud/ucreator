use std::{cell::{RefCell, UnsafeCell}, marker::PhantomData};

use crate::{Entity, UnsafeWorldPtrCell, World};

use super::SystemParam;

pub struct In<T>(pub T);

pub struct SystemId<I = (), O = ()> {
    entity: Entity,
    marker: PhantomData<fn(I) -> O>
}

impl<I, O> std::fmt::Debug for SystemId<I, O> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("SystemId")
            .field(&self.entity)
            .finish()
    }
}

impl<I, O> SystemId<I, O> {
    pub fn new(entity: Entity) -> Self {
        Self {
            entity,
            marker: PhantomData
        }
    }

    pub fn entity(&self) -> Entity {
        self.entity
    }
}

pub trait System: 'static {
    type In;
    type Out;

    fn init(&mut self, world: &mut World);

    fn run(&mut self, world: UnsafeWorldPtrCell, input: Self::In) -> Self::Out;

    fn is_initialized(&self) -> bool;
}

pub trait SystemFunction<M> {
    type In;
    type Out;
    type Param: SystemParam;

    fn run(&mut self, input: Self::In, params: <Self::Param as SystemParam>::Item<'_, '_>) -> Self::Out;
}

type SystemParamItem<'world, 'state, P> = <P as SystemParam>::Item<'world, 'state>;

macro_rules! impl_system_function {
    () => {
        impl<_O: 'static, _F: 'static> SystemFunction<fn() -> _O> for _F
        where
            for<'a> &'a mut _F: FnMut() -> _O {
            type In = ();
            type Out = _O;
            type Param = ();

            fn run(&mut self, _: Self::In, _: Self::Param) -> Self::Out {
                fn call<Out>(mut func: impl FnMut() -> Out) -> Out {
                    func()
                }

                call(self)
            }
        }

        impl<_I: 'static, _O: 'static, _F: 'static> SystemFunction<fn(In<_I>) -> _O> for _F
        where
            for<'a> &'a mut _F: FnMut(In<_I>) -> _O {
            type In = _I;
            type Out = _O;
            type Param = ();

            fn run(&mut self, input: Self::In, _: Self::Param) -> Self::Out {
                fn call<In, Out>(mut func: impl FnMut(In) -> Out, input: In) -> Out {
                    func(input)
                }

                call(self, In(input))
            }
        }
    };
    ($head:ident, $($tail:ident),*) => {
        #[allow(non_snake_case)]
        impl<_O: 'static, _F: 'static, $head: crate::system::SystemParam, $($tail: crate::system::SystemParam),*> SystemFunction<fn($head, $($tail),*) -> _O> for _F
        where
            for<'a> &'a mut _F:
                FnMut($head, $($tail),*) -> _O +
                FnMut(SystemParamItem<$head>, $(SystemParamItem<$tail>),*) -> _O {
            type In = ();
            type Out = _O;
            type Param = ($head, $($tail),*);

            fn run(&mut self, _: Self::In, params: SystemParamItem<($head, $($tail),*)>) -> Self::Out {
                fn call<Out, $head, $($tail),*>(mut func: impl FnMut($head, $($tail),*) -> Out, $head: $head, $($tail: $tail),*) -> Out {
                    func($head, $($tail),*)
                }

                let ($head, $($tail),*) = params;

                call(self, $head, $($tail),*)
            }
        }

        #[allow(non_snake_case)]
        impl<_I: 'static, _O: 'static, _F: 'static, $head: crate::system::SystemParam, $($tail: crate::system::SystemParam),*> SystemFunction<fn(In<_I>, $head, $($tail),*) -> _O> for _F
        where
            for<'a> &'a mut _F:
                FnMut(In<_I>, $head, $($tail),*) -> _O +
                FnMut(In<_I>, SystemParamItem<$head>, $(SystemParamItem<$tail>),*) -> _O {
            type In = _I;
            type Out = _O;
            type Param = ($head, $($tail),*);

            fn run(&mut self, input: Self::In, params: SystemParamItem<($head, $($tail),*)>) -> Self::Out {
                fn call<In, Out, $head, $($tail),*>(mut func: impl FnMut(In, $head, $($tail),*) -> Out, input: In, $head: $head, $($tail: $tail),*) -> Out {
                    func(input, $head, $($tail),*)
                }

                let ($head, $($tail),*) = params;

                call(self, In(input), $head, $($tail),*)
            }
        }
    }
}

pub trait IntoSystem<I, O, M> {
    type System: System<In = I, Out = O>;

    fn into_system(this: Self) -> Self::System;
}

impl<T: System> IntoSystem<T::In, T::Out, ()> for T {
    type System = T;

    fn into_system(this: Self) -> Self::System {
        this
    }
}

pub struct IsFunctionSystem;

impl<Marker, F> IntoSystem<F::In, F::Out, (IsFunctionSystem, Marker)> for F where Marker: 'static, F: SystemFunction<Marker> + 'static {
    type System = FunctionSystem<Marker, F>;

    fn into_system(this: Self) -> Self::System {
        Self::System {
            func: this,
            marker: PhantomData,
            state: None
        }
    }
}

pub struct FunctionSystem<M, F> where F: SystemFunction<M> {
    func: F,
    marker: PhantomData<fn() -> M>,
    state: Option<<F::Param as SystemParam>::State>
}

impl<M, F> System for FunctionSystem<M, F> where M: 'static, F: SystemFunction<M> + 'static {
    type In = F::In;
    type Out = F::Out;

    fn init(&mut self, world: &mut World) {
        self.state = Some(F::Param::init(world));
    }

    fn run(&mut self, world: UnsafeWorldPtrCell, input: Self::In) -> Self::Out {
        let params = F::Param::get(world, self.state.as_mut().unwrap());
        self.func.run(input, params)
    }

    fn is_initialized(&self) -> bool {
        self.state.is_some()
    }
}

uengine_utils::for_each_tuple_16!(impl_system_function);
