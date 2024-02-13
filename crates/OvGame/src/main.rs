#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use self::core::application::AppBuilder;

use rust_embed::RustEmbed;

pub mod core;

pub mod script;

#[derive(RustEmbed)]
#[folder = "assets"]
struct Asset;

fn main() {
    let app = AppBuilder::new().setPath(Asset).build();

    app.run();
}
