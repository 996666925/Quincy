use core::{application::Application, editor::Editor};

use QcWindowing::{event_loop::EventLoop, settings::WindowSettings};

mod components;
mod core;
mod managers;
mod pages;
mod panel;

fn main() {
    let app = Application::new();

    app.run();
}
