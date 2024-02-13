use std::any::Any;

use egui::Ui;
use OvWindowing::Window;

mod canvas;
pub use canvas::*;
mod button;
pub use button::*;
mod label;
pub use label::*;
mod textbox;
pub use textbox::*;
mod panel;
pub use panel::*;
mod image;
pub use image::*;

pub trait Component: Any {
    fn render(&mut self, ui: &mut Ui, window: &Window);
}
