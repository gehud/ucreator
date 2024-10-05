use std::{cell::Cell, process::Command};

use crate::{bundle::{Bundle, BundleInserter}, world::World};

use super::{EntityId, EntityLocation};

pub struct EntityMut<'w> {
    world: &'w mut World,
    id: EntityId,
    location: EntityLocation
}

impl<'w> EntityMut<'w> {
    pub(crate) fn new(world: &'w mut World, id: EntityId, location: EntityLocation) -> Self {
        Self {
            world,
            id,
            location
        }
    }

    pub fn insert<B: Bundle>(&mut self, bundle: B) -> &mut Self {
        let mut bundle_inserter = BundleInserter::new::<B>(
            &mut self.world,
            self.location.archetype_id()
        );

        self.location = bundle_inserter.insert(self.world, self.id, self.location, bundle);

        self
    }
}
