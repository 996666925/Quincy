#![allow(non_snake_case)]

use self::core::application::AppBuilder;

use rust_embed::RustEmbed;

use OvTools::time::clock::Clock;
use OvWindowing::{
    input::input_manager::InputManager, settings::window_settings::WindowSettings, Window,
};

pub mod core;

pub mod script;

#[derive(RustEmbed)]
#[folder = "assets"]
struct Asset;

fn main() {
    let app = AppBuilder::new().setPath(Asset).build();

    app.run();
}
