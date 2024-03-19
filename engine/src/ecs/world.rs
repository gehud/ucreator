use std::{any::TypeId, cell::UnsafeCell, collections::HashMap, hash::Hash, slice};

use super::{Entity, Error, Filter, Data, Storage, Table};

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum Group {
    Startup,
    Update
}

type SystemHandle = Box<dyn Fn(&mut World)>;

pub struct World {
    last_entity_index: usize,
    free_entities: Vec<usize>,
    systems: HashMap<Group, Vec<SystemHandle>>,
    components: Table
}

impl World {
    pub fn new() -> Self {
        Self {
            last_entity_index: 0usize,
            free_entities: Vec::new(),
            systems: HashMap::new(),
            components: HashMap::new()
        }
    }

    pub fn table(&self) -> &Table {
        &self.components
    }

    pub fn update(&mut self) {
        if !self.systems.contains_key(&Group::Update) {
            return;
        }

        let data = self.systems.get_mut(&Group::Update).unwrap().as_ptr() as *mut SystemHandle;
        let len = self.systems.get_mut(&Group::Update).unwrap().len();
        let systems = unsafe { slice::from_raw_parts_mut(data, len) };

        for system in systems {
            system(self);
        }
    }

    pub fn add_system(&mut self, group: Group, system: impl Fn(&mut World) + 'static) {
        if group == Group::Startup {
            system(self);
            return;
        }

        let systems = self.systems
            .entry(group)
            .or_insert(Vec::new());

        systems.push(Box::new(system));
    }

    pub fn add_component<T: 'static>(&mut self, entity: &Entity, data: T) -> Result<&mut T, Error> {
        let storage = self.components.entry(TypeId::of::<T>()).or_insert(Storage::new::<T>());
        storage.push::<T>(entity, data)?;
        Ok(storage.get_mut::<T>(entity)?)
    }

    pub fn entity_count(&self) -> usize {
        self.last_entity_index - self.free_entities.len() + 1usize
    }

    pub fn create_entity(&mut self) -> Result<Entity, Error> {
        let index = match self.free_entities.pop() {
            Some(value) => Ok(value),
            None => {
                if self.last_entity_index == usize::MAX {
                    return Err(Error::WorldOutOfBounds)
                }

                let value = self.last_entity_index;
                self.last_entity_index += 1;

                Ok(value)
            }
        }?;

        Ok(Entity::new(index))
    }
}
