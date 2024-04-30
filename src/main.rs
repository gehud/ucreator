#![allow(non_snake_case)]

use uengine::App;

mod layer;
use layer::CreatorLayer;

fn main() {
    App::new()
        .add_layer(CreatorLayer::default())
        .run();
}
