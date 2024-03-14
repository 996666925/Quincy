use std::any::Any;

use egui::{Ui, Vec2};
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
mod grid;
pub use grid::*;

use crate::core::ui_manager::UiManager;

pub trait Component: Any {
    fn render(&mut self, ui: &mut Ui, window: &Window);
}

pub trait PanelWindow: Any {

    fn active(&mut self){

    }

    fn disable(&mut self){

    }

    fn get_canvas(&mut self) -> &mut Canvas;

    fn get_size(&self) -> Vec2;
}
