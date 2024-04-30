use std::any::{type_name, TypeId};

use crate::System;

pub trait Group {
    fn rate(&mut self) -> f32;

    fn should_update(&mut self) -> bool;
}

pub trait SystemGroup {
    type System: System + 'static;
    type Group: Group + Default + 'static;

    fn system_id() -> TypeId {
        TypeId::of::<Self::System>()
    }

    fn system_name() -> &'static str {
        type_name::<Self::System>()
    }

    fn group_id() -> TypeId {
        TypeId::of::<Self::Group>()
    }

    fn group() -> impl Group {
        Self::Group::default()
    }
}

#[derive(Default)]
pub struct Simulation;

impl Group for Simulation {
    fn rate(&mut self) -> f32 {
        0.0
    }

    fn should_update(&mut self) -> bool {
        true
    }
}

#[derive(Default)]
pub struct Physics;

impl Group for Physics {
    fn rate(&mut self) -> f32 {
        0.02
    }

    fn should_update(&mut self) -> bool {
        true
    }
}

pub trait GroupChain {
    fn grow(chain: &mut Vec<TypeId>);
}

impl<T: Group + 'static> GroupChain for T {
    fn grow(chain: &mut Vec<TypeId>) {
        chain.push(TypeId::of::<T>());
    }
}

macro_rules! impl_system_chain {
    () => {};
    ($head:ident, $($tail:ident),*) => {
        impl<$head: GroupChain, $($tail: GroupChain),*> GroupChain for ($head, $($tail),*) {
            fn grow(chain: &mut Vec<TypeId>) {
                $head::grow(chain);
                $($tail::grow(chain);)*
            }
        }
    }
}

uengine_utils::for_each_tuple_16!(impl_system_chain);
