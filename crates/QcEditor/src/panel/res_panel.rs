use QcUI::core::context::UiContext;

use crate::{components::dock::DockView, core::context::Context};

use std::sync::Arc;

#[derive(Debug)]
pub struct ResPanel{
    context: Arc<Context>
}

impl DockView for ResPanel{
    fn render(&mut self, ctx: &mut UiContext, show_tab: bool) {
        
    }
}


impl ResPanel {
    pub fn new(context: Arc<Context>) -> Self {
        Self { context }
    }
}
