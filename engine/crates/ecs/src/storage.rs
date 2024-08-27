use std::{any::TypeId, collections::HashMap, ptr};

use uengine_any::TypeInfo;

use super::{Entity, Error};

pub struct Storage {
    info: TypeInfo,
    dense: Vec<u8>,
    len: usize,
    sparse: Vec<usize>
}

impl Storage {
    pub fn new<T: 'static>() -> Self {
        Self {
            info: TypeInfo::of::<T>(),
            dense: Vec::new(),
            len: 0usize,
            sparse: Vec::new()
        }
    }

    pub fn len(&self) -> usize {
        self.len
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

    pub fn push<T: 'static>(&mut self, entity: &Entity, data: T) -> Result<(), Error> {
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

        let data_index = match self.info.size() {
            0 => 0,
            value => self.dense.len() / value
        };

        self.sparse[entity.index()] = data_index;

        self.dense.resize(self.dense.len() + self.info.size(), 0u8);
        unsafe { ptr::write((self.dense.as_mut_ptr() as *mut T).wrapping_add(data_index), data); }

        self.len += 1;

        Ok(())
    }

    pub fn get_mut<T: 'static>(&mut self, entity: &Entity) -> Result<&mut T, Error> {
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

pub type Table = HashMap<TypeId, Storage>;
