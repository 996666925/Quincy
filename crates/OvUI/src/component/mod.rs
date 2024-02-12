use std::any::Any;

use egui::Ui;
use OvWindowing::Window;

mod canvas;
pub use canvas::*;
mod button;
pub use button::*;
mod label;
pub use label::*;
mod textBox;
pub use textBox::*;
mod panel;
pub use panel::*;

pub trait Component: Any {
    fn render(&mut self, ui: &mut Ui, window: &Window);
}
