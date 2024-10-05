use std::cell::Cell;

use crate::{archetype::Archetypes, component::{Component, Components}, entity::{Entities, EntityId}, storage::Storages, system::{IntoSystem, System, SystemError, SystemId}, Error};

#[derive(Component)]
#[component(storage = "sparse")]
pub struct RegisteredSystem<I, O> {
    system: Box<dyn System<In = I, Out = O>>
}

pub struct World {
    archetypes: Archetypes,
    components: Components,
    entities: Entities,
    storages: Storages,
    last_entity_index: usize,
}

impl World {
    pub fn new() -> Self {
        Self {
            archetypes: Archetypes::new(),
            components: Components::new(),
            entities: Entities::new(),
            storages: Storages::new(),
            last_entity_index: 0usize,
        }
    }

    pub fn archetypes(&self) -> &Archetypes {
        &self.archetypes
    }

    pub fn archetypes_mut(&mut self) -> &mut Archetypes {
        &mut self.archetypes
    }

    pub fn components(&self) -> &Components {
        &self.components
    }

    pub fn components_mut(&mut self) -> &mut Components {
        &mut self.components
    }

    pub fn entities(&self) -> &Entities {
        &self.entities
    }

    pub fn entities_mut(&mut self) -> &mut Entities {
        &mut self.entities
    }

    pub fn storages(&self) -> &Storages {
        &self.storages
    }

    pub fn storages_mut(&mut self) -> &mut Storages {
        &mut self.storages
    }

    pub fn create_entity(&mut self) -> Result<EntityId, Error> {
        if self.last_entity_index == usize::MAX {
            Err(Error::WorldOutOfBounds)
        } else {
            self.last_entity_index += 1;
            let entity = EntityId::new(self.last_entity_index);

            self.archetypes.insert_empty_entity(entity);

            Ok(entity)
        }
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

        let out = system.run(Cell::<World>::from_mut(self), input);

        self.add_component(entity, RegisteredSystem {
            system
        }).unwrap();

        Ok(out)
    }

    pub fn run_system<O: 'static>(&mut self, id: SystemId<(), O>) -> Result<O, SystemError<(), O>> {
        self.invoke_system(id, ())
    }
}
