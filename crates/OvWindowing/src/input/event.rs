use serde::Serialize;
use winit::event::{ElementState, MouseButton};

#[derive(Serialize)]
pub struct MouseEvent {
    pub state: ElementState,
    pub button: MouseButton,
    pub position: (f64, f64),
}

#[derive(Serialize)]
pub struct MouseMoveEvent {
    pub state: String,
    pub position: (f64, f64),
}
