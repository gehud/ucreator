#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#![allow(non_snake_case)]

use uengine_core::{App, Layer};
use uengine_ecs::{component::Component, In, Query, World};

mod scene;

#[derive(Component)]
struct A(u32);

#[derive(Component)]
struct B(f32);

struct MyLayer;

fn my_system(query: Query<&A>) {
    for a in query.iter() {
        println!("{}", a.0);
    }
}

impl Layer for MyLayer {
    fn on_create(&mut self, _: &mut App) {
        let mut world = World::new();
        let entity = world.create_entity().unwrap();
        world.add_component(entity, A(42u32)).unwrap();

        let system_id = world.register_system(my_system).unwrap();
        world.run_system(system_id).unwrap();
    }

    fn on_update(&mut self, _: &mut App) {

    }
}

fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");

    App::new()
        .add_layer(MyLayer)
        .run();

    // eframe::run_native(
    //     "UCreator",
    //     eframe::NativeOptions {
    //         viewport: eframe::egui::ViewportBuilder {
    //             maximized: Some(true),
    //             ..Default::default()
    //         },
    //         ..Default::default()
    //     },
    //     Box::new(|ctx| {
    //         let mut app = Box::new(CreatorLayer::default());
    //         app.on_create(ctx);
    //         app
    //     }))
}
