use serde::Serialize;
use winit::{
    event::{ElementState, MouseButton},
    keyboard::KeyCode,
};

#[derive(Serialize)]
pub struct KeyBoardEvent {
    pub key: KeyCode,
    pub state: ElementState,
}

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
