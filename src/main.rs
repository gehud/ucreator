#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#![allow(non_snake_case)]

use uengine_core::{App, Layer};
use uengine_ecs::{In, Query, World};

mod scene;

struct MyLayer;

fn my_system(In(mut counter): In<u32>) -> u32 {
    counter += 5;
    counter
}

impl Layer for MyLayer {
    fn on_create(&mut self, _: &mut App) {
        let mut world = World::new();
        let entity = world.create_entity().unwrap();
        world.add_component(&entity, 5u32).unwrap();

        let system_id = world.register_system(my_system).unwrap();
        let mut counter = 5u32;
        counter = world.invoke_system(system_id, counter).unwrap();
        println!("{}", counter)
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
