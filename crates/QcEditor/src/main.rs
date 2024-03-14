use core::{application::Application, editor::Editor, project_hub::ProjectHub};

use QcWindowing::{event_loop::EventLoop, settings::WindowSettings};

mod components;
mod core;
mod managers;

fn main() {
    let app = Application::new();

    app.run();
}
