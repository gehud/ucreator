use std::{cell::UnsafeCell, marker::PhantomData};

use super::{Data, Filter, Table, World};

pub struct Query<'this, D: Data, F: Filter = ()> {
    d: PhantomData<D>,
    f: PhantomData<F>,
    world: &'this mut World
}

impl<'this, D: Data, F: Filter> Query<'this, D, F> {
    pub fn new(world: &'this mut World) -> Self {
        Self {
            d: Default::default(),
            f: Default::default(),
            world
        }
    }

    pub fn for_each<'a>(&self, f: impl Fn(D::Type<'a>)) {
        if D::contains(self.world.table()) {
            let primary = D::primary(self.world.table());
            let c = self.world.table() as *const Table as *const UnsafeCell<Table>;
            primary.iter().for_each(|entity| {
                if D::matches(self.world.table(), &entity) && F::matches(self.world.table(), &entity) {
                    unsafe {
                        f(D::fetch(&*c, &entity));
                    }
                }
            });
        }
    }
}
