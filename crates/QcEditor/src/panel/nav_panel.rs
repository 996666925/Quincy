use std::sync::Arc;

use egui::Color32;
use QcUI::{
    component::{Label, UiNodeTrait},
    core::context::UiContext,
};

use crate::{components::dock::DockView, core::context::Context};

#[derive(Debug)]
pub struct NavPanel {
    context: Arc<Context>,
}

impl DockView for NavPanel {
    fn render(&mut self, ctx: &mut UiContext, show_tab: bool) {
        Label::default()
            .with_text("Quincy 游戏引擎")
            .with_color(Color32::WHITE)
            .render(ctx);
    }
}

impl NavPanel {
    pub fn new(context: Arc<Context>) -> Self {
        Self { context }
    }
}
