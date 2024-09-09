use uengine_cell::UnsafePtrCell;

use crate::{archetype::Archetypes, component::Component, storage::Storage, system::{IntoSystem, SystemError, SystemId}, Entity, Error, System};

#[derive(Component)]
#[component(storage = "sparse")]
pub struct RegisteredSystem<I, O> {
    system: Box<dyn System<In = I, Out = O>>
}

pub struct World {
    storage: Storage,
    last_entity_index: usize,
}

impl World {
    pub fn new() -> Self {
        Self {
            storage: Storage::new(),
            last_entity_index: 0usize,
        }
    }

    pub fn create_entity(&mut self) -> Result<Entity, Error> {
        if self.last_entity_index == usize::MAX {
            Err(Error::WorldOutOfBounds)
        } else {
            self.last_entity_index += 1;
            let entity = Entity::new(self.last_entity_index);

            self.storage.insert_empty_entity(entity);

            Ok(entity)
        }
    }

    pub fn add_component<C: Component>(&mut self, entity: Entity, component: C) -> Result<(), Error> {
        Ok((!self.storage.insert(entity, component).is_some()).then_some(()).ok_or(Error::ComponentAlreadyPresented)?)
    }

    pub fn get_component<C: Component>(&self, entity: &Entity) -> Result<&C, Error> {
        Ok(self.storage.get::<C>(entity).ok_or(Error::ComponentAlreadyPresented)?)
    }

    pub fn get_component_mut<C: Component>(&mut self, entity: &Entity) -> Result<&mut C, Error> {
        Ok(self.storage.get_mut::<C>(entity).ok_or(Error::ComponentAlreadyPresented)?)
    }

    pub fn remove_component<C: Component>(&mut self, entity: &Entity) -> Option<C> {
        self.storage.remove::<C>(entity)
    }

    pub fn register_system<I: 'static, O: 'static, M, S: IntoSystem<I, O, M> + 'static>(
        &mut self,
        system: S
    ) -> Result<SystemId<I, O>, Error> {
        let entity = self.create_entity()?;

        let mut system = Box::new(IntoSystem::into_system(system));
        system.init(self);

        self.add_component(
            entity,
            RegisteredSystem {
                system
            }
        )?;

        Ok(SystemId::new(entity))
    }

    pub fn invoke_system<I: 'static, O: 'static>(&mut self, id: SystemId<I, O>, input: I) -> Result<O, SystemError<I, O>> {
        let entity = id.entity();
        let RegisteredSystem { mut system } = match self.remove_component::<RegisteredSystem<I, O>>(&entity) {
            Some(value) => Ok(value),
            None => Err(SystemError::<I, O>::SystemNotRegistered(id)),
        }?;

        let out = system.run(self.into(), input);

        self.add_component(entity, RegisteredSystem {
            system
        }).unwrap();

        Ok(out)
    }

    pub fn run_system<O: 'static>(&mut self, id: SystemId<(), O>) -> Result<O, SystemError<(), O>> {
        self.invoke_system(id, ())
    }
}

pub type UnsafeWorldPtrCell<'a> = UnsafePtrCell<'a, World>;
