use std::sync::Arc;

use QcUI::core::context::UiContext;

use crate::{components::dock::DockView, core::context::Context};


#[derive(Debug)]
pub struct AttrPanel{
    context: Arc<Context>
}

impl DockView for AttrPanel{
    fn render(&mut self, ctx: &mut UiContext) {
        
    }
}

impl AttrPanel {
    pub fn new(context: Arc<Context>) -> Self {
        Self { context }
    }
}
