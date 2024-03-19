use std::{any::TypeId, cell::UnsafeCell, collections::HashMap, hash::Hash, slice};

use super::{Entity, Error, Storage};

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum Group {
    Startup,
    Update
}

type SystemHandle = Box<dyn Fn(&mut World)>;

pub type Table = HashMap<TypeId, Storage>;

pub struct World {
    last_entity_index: usize,
    free_entities: Vec<usize>,
    systems: HashMap<Group, Vec<SystemHandle>>,
    components: Table
}

pub trait Query {
    type Type<'a>;

    fn contains(components: &Table) -> bool;

    fn primary<'a>(components: &'a Table) -> &'a Storage;

    fn matches(components: &Table, entity: &Entity) -> bool;

    fn fetch<'a>(components: &'a UnsafeCell<Table>, entity: &Entity) -> Self::Type<'a>;
}

impl<T: 'static> Query for &mut T {
    type Type<'a> = &'a mut T;

    fn contains(components: &Table) -> bool {
        components.contains_key(&TypeId::of::<T>())
    }

    fn fetch<'a>(components: &'a UnsafeCell<Table>, entity: &Entity) -> <&'a mut T as Query>::Type<'a> {
        let storage = unsafe { &mut *components.get() };
        storage.get_mut(&TypeId::of::<T>()).unwrap().get_mut(entity).unwrap()
    }

    fn primary<'a>(components: &'a Table) -> &'a Storage {
        components.get(&TypeId::of::<T>()).unwrap()
    }

    fn matches(components: &Table, entity: &Entity) -> bool {
        components.get(&TypeId::of::<T>()).unwrap().contains(entity)
    }
}

macro_rules! for_each_tuple_ {
    ( $m:ident !! ) => (
        $m! { }
    );
    ( $m:ident !! $h:ident, $($t:ident,)* ) => (
        $m! { $h, $($t)* }
        for_each_tuple_! { $m !! $($t,)* }
    );
}

macro_rules! for_each_tuple {
    ( $m:ident ) => (
        for_each_tuple_! { $m !! A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, }
    );
}

macro_rules! impl_query {
    () => {};
    ($head:ident, $($tail:ident)*) => {
        impl<$head: Query, $($tail: Query),*> Query for ($head, $($tail),*)
        {
            type Type<'a> = (<$head as Query>::Type<'a>, $(<$tail as Query>::Type<'a>),*);

            fn contains(components: &Table) -> bool {
                $head::contains(components) $(&& $tail::contains(components))*
            }

            fn fetch<'a>(components: &'a UnsafeCell<Table>, entity: &Entity) -> <($head, $($tail),*) as Query>::Type<'a> {
                ($head::fetch(components, entity), $($tail::fetch(components, entity)),*)
            }

            fn primary<'a>(components: &'a Table) -> &'a Storage {
                $head::primary(components)
            }

            fn matches(components: &Table, entity: &Entity) -> bool {
                $head::matches(components, entity) $(&& $tail::matches(components, entity))*
            }
        }
    };
}

for_each_tuple!(impl_query);

impl World {
    pub fn new() -> Self {
        Self {
            last_entity_index: 0usize,
            free_entities: Vec::new(),
            systems: HashMap::new(),
            components: HashMap::new()
        }
    }

    pub fn for_each<'a, T: Query>(&'a mut self, f: impl Fn(T::Type<'a>)) {
        if T::contains(&self.components) {
            let primary = T::primary(&self.components);
            let c = &self.components as *const Table as *const UnsafeCell<Table>;
            primary.iter().for_each(|entity| {
                if T::matches(&self.components, &entity) {
                    unsafe {
                        f(T::fetch(&*c, &entity));
                    }
                }
            });
        }
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
