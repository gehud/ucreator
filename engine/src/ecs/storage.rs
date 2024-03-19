use std::{any::TypeId, mem::size_of, ptr};

use super::{Entity, Error, Result};

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

pub struct Storage {
    info: TypeInfo,
    dense: Vec<u8>,
    sparse: Vec<usize>
}

impl Storage {
    pub fn new<T: 'static>() -> Self {
        Self {
            info: TypeInfo::of::<T>(),
            dense: Vec::new(),
            sparse: Vec::new()
        }
    }

    pub fn contains(&self, entity: &Entity) -> bool {
        match self.sparse.get(entity.index()) {
            Some(result) => {
                if *result == usize::MAX {
                    return false
                }

                true
            },
            None => {
                false
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = Entity> + '_ {
        self.sparse.iter().map_while(|index| {
            if *index == usize::MAX {
                return None
            }

            Some(Entity::new(*index))
        })
    }

    pub fn capacity(&self) -> usize {
        self.sparse.len()
    }

    pub fn push<T: 'static>(&mut self, entity: &Entity, data: T) -> Result<()> {
        if cfg!(debug_assertions) {
            if TypeId::of::<T>() != *self.info.id() {
                return Err(Error::TypeNotPresented)
            }
        }

        match self.sparse.get(entity.index()) {
            Some(value) => {
                if *value == usize::MAX {
                    return Err(Error::EntityAlreadyPresented)
                }

                Ok(())
            },
            None => {
                let old_len = self.sparse.len();
                let new_len = old_len + (entity.index() + 1usize - old_len);

                self.sparse.resize(new_len, usize::MAX);

                Ok(())
            }
        }?;

        let data_index = self.dense.len() / self.info.size();

        self.sparse[entity.index()] = data_index;

        self.dense.resize(self.dense.len() + self.info.size(), 0u8);
        unsafe { ptr::write((self.dense.as_mut_ptr() as *mut T).wrapping_add(data_index), data); }

        Ok(())
    }

    pub fn get_mut<T: 'static>(&mut self, entity: &Entity) -> Result<&mut T> {
        let index = match self.sparse.get(entity.index()) {
            Some(result) => {
                if *result == usize::MAX {
                    return Err(Error::EntityNotPresented)
                }

                Ok(result)
            },
            None => {
                Err(Error::EntityNotPresented)
            }
        }?;

        Ok(unsafe {&mut *(self.dense.as_mut_ptr() as *mut T).wrapping_add(*index) })
    }
}
