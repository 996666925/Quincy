use core::project_hub::ProjectHub;

mod core;
mod components;
mod managers;

fn main() {
    let hub = ProjectHub::new();

    hub.run();
}
