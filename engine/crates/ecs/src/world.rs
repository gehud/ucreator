use std::{any::TypeId, collections::HashMap, mem::MaybeUninit, slice};

use uengine_any::TypeInfo;

use crate::{archetype::ArchetypeKey, system::{IntoSystem, SystemError, SystemId}, Archetype, Entity, Error, System};

struct ArchetypeSearchNode {
    type_id: TypeId,
    index: usize,
    children: Vec<usize>
}

struct ArchetypeSearchTree {
    nodes: Vec<ArchetypeSearchNode>
}

pub struct RegisteredSystem<I, O> {
    system: Box<dyn System<In = I, Out = O>>
}

pub struct World {
    archetypes: Vec<Archetype>,
    archetype_keys: HashMap<ArchetypeKey, usize>,
    archetype_search_tree: ArchetypeSearchNode,
    entities: HashMap<Entity, usize>,
    last_entity_index: usize,
}

impl World {
    pub fn new() -> Self {
        let mut world = Self {
            archetypes: Vec::new(),
            archetype_keys: HashMap::new(),
            archetype_search_tree: ArchetypeSearchNode {
                type_id: TypeId::of::<()>(),
                index: 0usize,
                children: Vec::new()
            },
            entities: HashMap::new(),
            last_entity_index: 0usize,
        };

        world.add_archetype(crate::archetype!()).unwrap();

        world
    }

    fn add_archetype(&mut self, archetype: Archetype) -> Result<(), Error> {
        let key = archetype.extract_key();

        let archetype_index = self.archetypes.len();

        // TODO: insert searching nodes

        match self.archetype_keys.insert(key, archetype_index) {
            Some(_) => Err(Error::ArchetypeAllreadyPresented),
            None => Ok(()),
        }?;

        self.archetypes.push(archetype);

        Ok(())
    }

    pub fn get_archetype(&self, key: &ArchetypeKey) -> Result<&Archetype, Error> {
        Ok(self.archetypes.get(*self.archetype_keys.get(key).ok_or(Error::ArchetypeNotPresented)?).unwrap())
    }

    pub fn create_entity(&mut self) -> Result<Entity, Error> {
        if self.last_entity_index == usize::MAX {
            Err(Error::WorldOutOfBounds)
        } else {
            self.last_entity_index += 1;
            let entity = Entity::new(self.last_entity_index);

            let archetype = self.archetypes.get_mut(0usize).unwrap();

            archetype.add_row(entity)?;
            self.entities.insert(entity, 0usize);

            Ok(entity)
        }
    }

    fn find_suitable_archetype(&self, old_archetype_index: &usize, type_id: &TypeId) -> Option<usize> {
        for (index, archetype) in self.archetypes.iter().enumerate() {
            if archetype.contains_types_of(self.get_archetype_by_index(old_archetype_index).unwrap()) && archetype.contains_type(type_id) {
                return Some(index)
            }
        }

        return None;
    }

    fn find_entity_archetype(&self, entity: &Entity) -> Option<usize> {
        self.entities.get(entity).copied()
    }

    fn get_entity_archetype_index(&self, entity: &Entity) -> Result<usize, Error> {
        Ok(self.find_entity_archetype(entity).ok_or(Error::EntityNotPresented)?)
    }

    fn get_archetype_by_index(&self, index: &usize) -> Option<&Archetype> {
        self.archetypes.get(*index)
    }

    fn get_archetype_by_index_mut(&mut self, index: &usize) -> Option<&mut Archetype> {
        self.archetypes.get_mut(*index)
    }

    pub fn get_or_create_suitable_archetype(&mut self, old_archetype_index: &usize, type_info: &TypeInfo) -> Result<usize, Error> {
        if self.get_archetype_by_index(old_archetype_index).unwrap().contains_type(type_info.id()) {
            Err(Error::TypeAlreadyPresented)
        } else {
            let suitable_archetype_index = self.find_suitable_archetype(old_archetype_index, type_info.id());

            let archetype_index = match suitable_archetype_index {
                Some(value) => value,
                None => {
                    let new_archetype = self.get_archetype_by_index(old_archetype_index)
                        .unwrap()
                        .new_with_type(&type_info)?;

                    self.add_archetype(new_archetype)?;
                    self.archetypes.len() - 1usize
                },
            };

            Ok(archetype_index)
        }
    }

    pub fn add_component<T: 'static>(&mut self, entity: &Entity, component: T) -> Result<(), Error> {
        let type_info = TypeInfo::of::<T>();

        let old_archetype_index = self.get_entity_archetype_index(entity)?;
        let suitable_archetype_index = self.get_or_create_suitable_archetype(&old_archetype_index, &type_info)?;
        *self.entities.get_mut(entity).unwrap() = suitable_archetype_index;

        let old_archetype = self.get_archetype_by_index(&old_archetype_index).unwrap() as *const Archetype as *mut Archetype;
        let new_archetype = self.get_archetype_by_index(&suitable_archetype_index).unwrap() as *const Archetype as *mut Archetype;

        unsafe { new_archetype.as_mut().unwrap().add_row(*entity)? };

        let old_row = unsafe { old_archetype.as_mut() }.unwrap().get_row(entity)?;
        let new_row = unsafe { new_archetype.as_mut().unwrap() }.get_row_mut(entity)?;

        let mut src_start = 0usize;
        let mut dst_start = 0usize;
        let mut new_dst_start = 0usize;
        for info in unsafe { new_archetype.as_ref() }.unwrap().types() {
            if *info == type_info {
                new_dst_start = dst_start;
                dst_start += info.size();
                continue;
            }

            let src_end = src_start + info.size();
            let dst_end = dst_start + info.size();
            new_row[dst_start..dst_end]
                .copy_from_slice(&old_row[src_start..src_end]);

            src_start += info.size();
            dst_start += info.size();
        }

        unsafe {
            new_row.as_mut_ptr().add(new_dst_start).cast::<T>()
                .write(component);
        }

        unsafe { old_archetype.as_mut() }.unwrap().remove_row(entity)?;

        Ok(())
    }

    pub fn get_component<T: 'static>(&self, entity: &Entity) -> Result<&T, Error> {
        let archetype_index = self.find_entity_archetype(entity).ok_or(Error::EntityNotPresented)?;
        self.get_archetype_by_index(&archetype_index).unwrap().get_component(entity)
    }

    pub fn get_component_mut<T: 'static>(&mut self, entity: &Entity) -> Result<&mut T, Error> {
        let archetype_index = self.find_entity_archetype(entity).ok_or(Error::EntityNotPresented)?;
        self.get_archetype_by_index_mut(&archetype_index).unwrap().get_component_mut(entity)
    }

    pub fn register_system<I: 'static, O: 'static, M, S: IntoSystem<I, O, M> + 'static>(
        &mut self,
        system: S
    ) -> Result<SystemId<I, O>, Error> {
        let entity = self.create_entity()?;

        let mut system = Box::new(IntoSystem::into_system(system));
        system.init(self);

        self.add_component(
            &entity,
            RegisteredSystem {
                system
            }
        )?;

        Ok(SystemId::new(entity))
    }

    pub fn invoke_system<I: 'static, O: 'static>(&mut self, id: SystemId<I, O>, input: I) -> Result<O, SystemError<I, O>> {
        let RegisteredSystem { system } = match self.get_component_mut::<RegisteredSystem<I, O>>(&id.entity()) {
            Ok(value) => Ok(value),
            Err(_) => Err(SystemError::<I, O>::SystemNotRegistered(id)),
        }?;

        Ok(system.run(input))
    }

    pub fn run_system<O: 'static>(&mut self, id: SystemId<(), O>) -> Result<O, SystemError<(), O>> {
        self.invoke_system(id, ())
    }
}
