use QcCore::ecs::component::Updated;
use QcMacros::{Comp, Component};

// #[derive(Debug, Component, Clone)]
pub struct Example;

impl Updated for Example {
    fn update(&mut self, dt: f32) {
        println!("dt:{}", dt);
    }
}
