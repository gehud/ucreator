use std::{any::TypeId, mem::MaybeUninit, slice};

use crate::component::{Component, ComponentId, Components, StoragePolicy};

pub trait Bundle: 'static {
    fn get_component_ids(
        components: &Components,
        func: &mut impl FnMut(ComponentId)
    );

    fn get_component_data(
        self,
        func: &mut impl FnMut(StoragePolicy, &[MaybeUninit<u8>])
    );
}

impl<C: Component> Bundle for C {
    fn get_component_ids(
        components: &Components,
        func: &mut impl FnMut(ComponentId)
    ) {
        func(components.get_component_id::<C>().expect("the component is not registered"))
    }

    fn get_component_data(
        self,
        func: &mut impl FnMut(StoragePolicy, &[MaybeUninit<u8>])
    ) {
        func(C::STORAGE_POLICY, MaybeUninit::new(self).as_bytes())
    }
}

macro_rules! impl_bundle {
    ($(($name:ident, $bundle:ident)),*) => {
        impl<$($name: Bundle),*> Bundle for ($($name,)*) {
            fn get_component_ids(
                _components: &Components,
                _func: &mut impl FnMut(ComponentId)
            ) {
                $(
                    $name::get_component_ids(_components, _func);
                )*
            }

            fn get_component_data(
                self,
                _func: &mut impl FnMut(StoragePolicy, &[MaybeUninit<u8>])
            ) {
                let ($($bundle,)*) = self;
                $(
                    $bundle.get_component_data(_func);
                )*
            }
        }
    };
}

uengine_utils::all_tuples!(impl_bundle, 0, 16, B, bundle);
