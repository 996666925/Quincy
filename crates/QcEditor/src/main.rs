use core::project_hub::ProjectHub;

mod core;
mod components;

fn main() {
    let hub = ProjectHub::new();

    hub.run();
}
