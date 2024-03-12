use std::any::Any;

use egui::Ui;
use QcWindowing::Window;

mod widget;
pub use widget::*;
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
mod image_loader;
pub use image_loader::*;

use crate::core::ui_manager::UiManager;

pub trait Component: Any {
    fn render(&mut self, ui: &mut Ui, window: &Window);
}

pub trait PanelWindow: Any {
    fn show(&mut self, window: &Window, ui: &mut UiManager) {
        ui.render(window, &mut vec![self.get_canvas()])
    }
    fn update(&mut self, ui: &mut UiManager) {
        ui.update_not_js(vec![self.get_canvas()]);
    }

    fn get_canvas(&mut self) -> &mut Canvas;
}
