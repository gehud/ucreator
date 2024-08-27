use std::{any::TypeId, mem::size_of};

#[derive(Hash, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TypeInfo {
    id: TypeId,
    size: usize
}

impl TypeInfo {
    pub fn of<T: 'static>() -> Self {
        Self {
            id: TypeId::of::<T>(),
            size: size_of::<T>()
        }
    }

    pub fn id(&self) -> &TypeId {
        &self.id
    }

    pub fn size(&self) -> &usize {
        &self.size
    }
}
