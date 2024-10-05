#![feature(maybe_uninit_as_bytes)]

mod type_info;
pub use type_info::TypeInfo;

mod vec;
pub use vec::AnyVec;
