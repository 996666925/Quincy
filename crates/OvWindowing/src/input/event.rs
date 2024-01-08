use serde::Serialize;
use winit::event::{ElementState, MouseButton};

#[derive(Serialize)]
pub struct VirtualMouse {
    pub state: ElementState,
    pub button: MouseButton,
    pub position: (f64, f64),
}
