use std::any::Any;

use egui::Ui;
use OvWindowing::Window;

mod button;
pub use button::*;
mod label;
pub use label::*;
mod input;
pub use input::*;
pub trait Component: Any {
    fn render(&mut self, ui: &mut Ui, window: &Window);
}
