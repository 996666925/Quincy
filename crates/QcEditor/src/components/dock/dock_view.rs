use std::fmt::Debug;

use QcUI::core::context::UiContext;

pub trait DockView: Debug {
    fn render(&mut self, ctx: &mut UiContext);
}
