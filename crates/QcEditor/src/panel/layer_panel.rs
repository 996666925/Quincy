use std::sync::Arc;

use QcUI::core::context::UiContext;

use crate::{components::dock::DockView, core::context::Context};

#[derive(Debug)]
pub struct LayerPanel {

    context: Arc<Context>
}

impl DockView for LayerPanel {
    fn render(&mut self, ctx: &mut UiContext) {}
}

impl LayerPanel {
    pub fn new(context: Arc<Context>) -> Self {
        Self { context }
    }
}
