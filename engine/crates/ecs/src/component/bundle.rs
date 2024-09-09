use super::ComponentInfo;

pub trait Bundle {
    fn info() -> Vec<ComponentInfo>;
}

macro_rules! impl_bundle {
    () => {
        impl<T: crate::component::Component> Bundle for T {
            fn info() -> Vec<ComponentInfo> {
                vec!(ComponentInfo::of::<T>())
            }
        }
    };
    ($head:ident, $($tail:ident),*) => {
        impl<$head: crate::component::Component, $($tail: crate::component::Component),*> Bundle for ($head, $($tail),*) {
            fn info() -> Vec<ComponentInfo> {
                vec!(ComponentInfo::of::<$head>(), $(ComponentInfo::of::<$tail>()),*)
            }
        }
    };
}

uengine_utils::for_each_tuple_16!(impl_bundle);
