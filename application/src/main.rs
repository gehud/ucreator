#![allow(non_snake_case)]

use uengine::{ecs::{Group, Query, With, Without, World}, log::trace};

#[derive(Debug, Default)]
struct Position {
    pub x: f32,
    pub y: f32
}

#[derive(Debug, Default)]
struct Rotation {
    pub value: f32
}

fn my_startup_system(world: &mut World) {
    let entity = world.create_entity().unwrap();
    let position = world.add_component(&entity, Position::default()).unwrap();
    position.x = 1.0;
    position.y = 1.0;

    world.add_component(&entity, Rotation::default()).unwrap();
}

fn my_update_system(world: &mut World) {
    let query = Query::<&mut Position>::new(world);
    query.for_each(|position| {
        trace!("AHAHAHA");
    })
}

fn main() {
    let mut world = World::new();
    world.add_system(Group::Startup, my_startup_system);
    world.add_system(Group::Update, my_update_system);

    loop {
        world.update();
    }
}
