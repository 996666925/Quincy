use std::sync::Arc;

use QcUI::core::context::UiContext;

use crate::{components::dock::DockView, core::context::Context};



#[derive(Debug)]
pub struct ResPreviewPanel{
    context: Arc<Context>
}

impl DockView for ResPreviewPanel{
    fn render(&mut self, ctx: &mut UiContext) {
        
    }
}

impl ResPreviewPanel {
    pub fn new(context: Arc<Context>) -> Self {
        Self { context }
    }
}
