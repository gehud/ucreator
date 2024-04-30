use std::{any::{type_name, TypeId}, collections::HashMap};

use crate::{group::GroupChain, system::SystemChain, Context, Entity, Error, Group, Result, Storage, System, SystemCreation, SystemDestruction, SystemGroup, Table};

trait TrySystemCreation<T: System> {
    fn invoke(system: &mut T, context: &mut Context);
}

impl<T: System> TrySystemCreation<T> for T {
    default fn invoke(_: &mut T, _: &mut Context) {

    }
}

impl<T: System + SystemCreation> TrySystemCreation<T> for T {
    fn invoke(system: &mut T, context: &mut Context) {
        system.on_create(context);
    }
}

trait TrySystemDestruction<T: System> {
    fn invoke(system: &mut T, context: &mut Context);
}

impl<T: System> TrySystemDestruction<T> for T {
    default fn invoke(_: &mut T, _: &mut Context) {

    }
}

impl<T: System + SystemDestruction> TrySystemDestruction<T> for T {
    fn invoke(system: &mut T, context: &mut Context) {
        system.on_destroy(context);
    }
}

struct SystemHandle {
    system: Box<dyn System>,
    context: Context
}

impl SystemHandle {
    pub fn new(system: Box<dyn System>, context: Context) -> Self {
        Self {
            system,
            context
        }
    }
}

struct GroupSystems {
    group: Box<dyn Group>,
    order: Vec<TypeId>,
    registry: HashMap<TypeId, SystemHandle>
}

impl GroupSystems {
    pub fn new(group: Box<dyn Group>) -> Self {
        Self {
            group,
            order: Vec::new(),
            registry: HashMap::new()
        }
    }

    pub fn update(&mut self) {
        for system_id in &self.order {
            let handle = self.registry.get_mut(system_id).unwrap();
            handle.system.on_update(&mut handle.context);
        }
    }
}

pub struct World {
    last_entity_index: usize,
    free_entities: Vec<usize>,
    components: Table,
    groups: HashMap<TypeId, GroupSystems>,
    group_order: Vec<TypeId>,
    system_groups: HashMap<TypeId, TypeId>
}

impl World {
    pub fn new() -> Self {
        Self {
            last_entity_index: 0usize,
            free_entities: Vec::new(),
            components: HashMap::new(),
            groups: HashMap::new(),
            group_order: Vec::new(),
            system_groups: HashMap::new()
        }
    }

    pub fn table(&self) -> &Table {
        &self.components
    }

    pub fn update(&mut self) {
        for group_id in &self.group_order {
            let group = self.groups.get_mut(group_id).unwrap();
            group.update();
        }
    }

    pub fn add_component<T: 'static>(&mut self, entity: &Entity, data: T) -> Result<&mut T> {
        let storage = self.components.entry(TypeId::of::<T>()).or_insert(Storage::new::<T>());
        storage.push::<T>(entity, data)?;
        Ok(storage.get_mut::<T>(entity)?)
    }

    pub fn entity_count(&self) -> usize {
        self.last_entity_index - self.free_entities.len() + 1usize
    }

    pub fn create_entity(&mut self) -> Result<Entity> {
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

    fn register_system_by(&mut self, system_id: &TypeId, group_id: &TypeId, group: fn() -> Box<dyn Group>, system_name: fn() -> &'static str) -> Result<()> {
        let container = self.groups.entry(*group_id).or_insert_with(|| {
            self.group_order.push(*group_id);
            GroupSystems::new(group())
        });

        if container.order.contains(system_id) {
            return Err(Error::SystemAlreadyRegistered(system_name()));
        }

        self.system_groups.insert(*system_id, *group_id);

        container.order.push(*system_id);

        Ok(())
    }

    pub fn register_system<S: SystemGroup + 'static>(&mut self) -> Result<()> {
        let system_id = S::system_id();
        let group_id = S::group_id();

        self.register_system_by(&system_id, &group_id, || Box::new(S::group()), || S::system_name())
    }

    pub fn create_system<S: System + Default + 'static>(&mut self) -> Result<()> {
        let world = self as *mut World;

        let group_id = self.system_groups.get(&TypeId::of::<S>())
            .ok_or(Error::SystemNotRegistered(type_name::<S>()))?;

        let system_id = &TypeId::of::<S>();

        let container = self.groups.get_mut(group_id)
            .ok_or(Error::GroupNotRegistered)?;

        match container.registry.insert(*system_id, SystemHandle::new(Box::new(S::default()), Context::new(world))) {
            None => {
                container.order.push(*system_id);
                Ok(())
            },
            Some(_) => {
                Err(Error::SystemAlreadyPresented(type_name::<S>()))
            }
        }?;

        let handle = container.registry.get_mut(system_id).unwrap();

        let system = unsafe { &mut *(handle.system.as_mut() as *mut dyn System as *mut S) };
        <S as TrySystemCreation::<S>>::invoke(system, &mut handle.context);

        Ok(())
    }

    pub fn sort_systems<S: SystemChain + 'static>(&mut self) {

    }

    pub fn sort_groups<G: GroupChain + 'static>(&mut self) {

    }
}
