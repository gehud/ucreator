use std::{any::TypeId, hash::Hash, mem::{needs_drop, size_of, MaybeUninit}};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TypeInfo {
    id: TypeId,
    size: usize,
    drop: Option<unsafe fn(*mut MaybeUninit<u8>)>
}

impl Hash for TypeInfo {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl TypeInfo {
    pub fn of<T: 'static>() -> Self {
        Self {
            id: TypeId::of::<T>(),
            size: size_of::<T>(),
            drop: needs_drop::<T>().then_some(Self::drop_in_place_as::<T>)
        }
    }

    pub fn id(&self) -> &TypeId {
        &self.id
    }

    pub fn size(&self) -> usize {
        self.size
    }

    #[inline]
    unsafe fn drop_in_place_as<T>(ptr: *mut MaybeUninit<u8>) {
        ptr.cast::<T>().drop_in_place();
    }
}
