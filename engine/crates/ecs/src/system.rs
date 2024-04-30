use std::any::TypeId;

use crate::World;

pub struct Context {
    world: *mut World
}

impl Context {
    pub fn new(world: *mut World) -> Self {
        Self {
            world
        }
    }

    pub fn world(&mut self) -> &mut World {
        unsafe {
            &mut *self.world
        }
    }
}

pub trait SystemCreation {
    fn on_create(&mut self, context: &mut Context);
}

pub trait System {
    fn on_update(&mut self, context: &mut Context);
}

pub trait SystemDestruction {
    fn on_destroy(&mut self, context: &mut Context);
}

pub trait SystemChain {
    fn grow(chain: &mut Vec<TypeId>);
}

impl<T: System + 'static> SystemChain for T {
    fn grow(chain: &mut Vec<TypeId>) {
        chain.push(TypeId::of::<T>());
    }
}

macro_rules! impl_system_chain {
    () => {};
    ($head:ident, $($tail:ident),*) => {
        impl<$head: SystemChain, $($tail: SystemChain),*> SystemChain for ($head, $($tail),*) {
            fn grow(chain: &mut Vec<TypeId>) {
                $head::grow(chain);
                $($tail::grow(chain);)*
            }
        }
    }
}

uengine_utils::for_each_tuple_16!(impl_system_chain);
