#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#![allow(non_snake_case)]

mod layer;
use layer::CreatorLayer;

fn main() -> eframe::Result<()> {
    std::env::set_var("RUST_BACKTRACE", "1");
    eframe::run_native(
        "UCreator",
        eframe::NativeOptions {
            viewport: eframe::egui::ViewportBuilder {
                maximized: Some(true),
                ..Default::default()
            },
            ..Default::default()
        },
        Box::new(|_| {
            let mut app = Box::new(CreatorLayer::default());
            app.on_create();
            app
        }))
}
