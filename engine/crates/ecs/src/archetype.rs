use std::{any::{Any, TypeId}, mem::MaybeUninit, slice};

use uengine_any::TypeInfo;

use crate::{Entity, Error};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArchetypeKey {
    types: Vec<TypeInfo>
}

impl ArchetypeKey {
    pub fn new(mut types: Vec<TypeInfo>) -> Self {
        types.sort();

        Self {
            types
        }
    }

    pub fn add_type(&mut self, type_info: TypeInfo) -> Result<(), Error> {
        match self.types.binary_search(&type_info) {
            Ok(index) => {
                self.types.insert(index, type_info);
                Ok(())
            },
            Err(_) => Err(Error::TypeAlreadyPresented),
        }
    }

    pub fn types(&self) -> &Vec<TypeInfo> {
        &self.types
    }
}

pub struct Archetype {
    entities: Vec<Entity>,
    types: Vec<TypeInfo>,
    row_size: usize,
    components: Vec<MaybeUninit<u8>>
}

impl Archetype {
    pub fn new(types: Vec::<TypeInfo>) -> Self {
        Self {
            entities: Vec::new(),
            row_size: types.iter().map(|item| item.size()).sum(),
            types,
            components: Vec::new()
        }
    }

    pub fn extract_key(&self) -> ArchetypeKey {
        ArchetypeKey::new(self.types.clone())
    }

    pub fn new_with_type(&self, new_type: &TypeInfo) -> Result<Self, Error> {
        let mut archetype = Self {
            entities: Vec::new(),
            row_size: self.row_size,
            types: self.types.clone(),
            components: Vec::new()
        };

        match archetype.types.binary_search(new_type) {
            Ok(_) => Err(Error::TypeAlreadyPresented),
            Err(pos) => {
                archetype.types.insert(pos, *new_type);
                archetype.row_size += new_type.size();
                Ok(())
            },
        }?;

        Ok(archetype)
    }

    pub fn contains_types_of(&self, other: &Archetype) -> bool {
        self.types.iter().all(|my_item| other.types.iter().any(|other_item| other_item.id() == my_item.id()))
    }

    pub fn contains_type(&self, type_id: &TypeId) -> bool {
        self.types.binary_search_by(|info| info.type_id().cmp(type_id)).is_ok()
    }

    pub fn is_subarchetype_of(&self, other: &Archetype) -> bool {
        self.contains_types_of(other)
    }

    pub fn types(&self) -> &Vec<TypeInfo> {
        &self.types
    }

    pub fn get_row(&self, entity: &Entity) -> Result<&[MaybeUninit<u8>], Error> {
        if !self.entities.contains(entity) {
            Err(Error::EntityNotPresented)
        } else {
            let entity_index = self.entities.iter().position(|item| item == entity).unwrap();
            let start_index = entity_index * self.row_size;
            let end_index = start_index + self.row_size;

            Ok(&self.components[start_index..end_index])
        }
    }

    pub fn get_row_mut(&mut self, entity: &Entity) -> Result<&mut [MaybeUninit<u8>], Error> {
        if !self.entities.contains(entity) {
            Err(Error::EntityNotPresented)
        } else {
            let entity_index = self.entities.iter().position(|item| item == entity).unwrap();
            let start_index = entity_index * self.row_size;
            let end_index = start_index + self.row_size;

            Ok(&mut self.components[start_index..end_index])
        }
    }

    pub fn add_row(&mut self, entity: Entity) -> Result<(), Error> {
        if self.entities.contains(&entity) {
            Err(Error::EntityAlreadyPresented)
        } else {
            self.entities.push(entity);

            let old_len = self.components.len();
            self.components.resize(old_len + self.row_size, MaybeUninit::uninit());

            Ok(())
        }
    }

    pub fn remove_row(&mut self, entity: &Entity) -> Result<(), Error> {
        if !self.entities.contains(&entity) {
            Err(Error::EntityNotPresented)
        } else {
            let entity_index = self.entities.iter().position(|item| item == entity).unwrap();
            let start_index = entity_index * self.row_size;
            let end_index = start_index + self.row_size;

            self.components.drain(start_index..end_index);

            self.entities.remove(entity_index);

            Ok(())
        }
    }

    pub fn set_component<T: 'static>(&mut self, entity: &Entity, component: T) -> Result<(), Error> {
        let type_info = &TypeInfo::of::<T>();

        let ptr = self.get_component_ptr_mut(entity, type_info)?;

        unsafe { ptr.cast::<T>().write(component) };

        Ok(())
    }

    pub fn get_component<T: 'static>(&self, entity: &Entity) -> Result<&T, Error> {
        let type_info = &TypeInfo::of::<T>();

        let ptr = self.get_component_ptr(entity, type_info)?;

        let data = unsafe {
            ptr.cast::<T>().as_ref().unwrap()
        };

        Ok(data)
    }

    pub fn get_component_mut<T: 'static>(&mut self, entity: &Entity) -> Result<&mut T, Error> {
        let type_info = &TypeInfo::of::<T>();

        let ptr = self.get_component_ptr_mut(entity, type_info)?;

        let data = unsafe {
            ptr.cast::<T>().as_mut().unwrap()
        };

        Ok(data)
    }

    fn get_component_position(&self, type_info: &TypeInfo) -> Option<usize> {
        self.types
            .iter()
            .position(|item| item == type_info)
    }

    fn get_component_offset(&self, type_info: &TypeInfo) -> Option<usize> {
        let position = self.get_component_position(type_info)?;

        let offset = self.types[..position]
            .iter()
            .map(|item| item.size())
            .sum();

        Some(offset)
    }

    fn get_component_ptr(&self, entity: &Entity, type_info: &TypeInfo) -> Result<*const MaybeUninit<u8>, Error> {
        let offset = self.get_component_offset(type_info)
            .ok_or(Error::TypeNotPresented)?;

        let row = self.get_row(entity)?;

        let ptr = unsafe { row.as_ptr().add(offset) };

        Ok(ptr)
    }

    fn get_component_ptr_mut(&mut self, entity: &Entity, type_info: &TypeInfo) -> Result<*mut MaybeUninit<u8>, Error> {
        if !self.entities.contains(entity) {
            Err(Error::EntityNotPresented)
        } else {
            let offset = self.get_component_offset(type_info)
            .ok_or(Error::TypeNotPresented)?;

            let row = self.get_row_mut(entity)?;
            let ptr = unsafe { row.as_mut_ptr().add(offset) };

            Ok(ptr)
        }
    }
}

#[macro_export]
macro_rules! __ecs_archetype__ {
    ($($t:ident),*) => {
        Archetype::new(vec!($(uengine_any::TypeInfo::of::<$t>()),*))
    };
}
