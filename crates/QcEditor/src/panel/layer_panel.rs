use std::sync::Arc;
use crate::components::nestable::HierarchicalDragAndDrop;
use QcUI::core::context::UiContext;

use crate::{components::dock::DockView, core::context::Context};

#[derive(Debug)]
pub struct LayerPanel {
    context: Arc<Context>,
    list: HierarchicalDragAndDrop,
}

impl DockView for LayerPanel {
    fn render(&mut self, ctx: &mut UiContext, show_tab: bool) {
        self.list.ui(ctx.ui)
    }
}

impl LayerPanel {
    pub fn new(context: Arc<Context>) -> Self {
        Self {
            context,
            list: HierarchicalDragAndDrop::default(),
        }
    }
}
