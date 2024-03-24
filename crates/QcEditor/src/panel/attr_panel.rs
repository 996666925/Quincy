#![feature(downcast_unchecked)]

use std::sync::Arc;

use egui::Color32;
use QcCore::ecs::component::Component;
use QcUI::{
    component::{Label, UiNodeTrait},
    core::context::UiContext,
};

use crate::{components::dock::DockView, core::context::Context, inspector::InspectorTrait};

#[derive(Debug)]
pub struct AttrPanel {
    context: Arc<Context>,
}

impl DockView for AttrPanel {
    fn render(&mut self, ctx: &mut UiContext, show_tab: bool) {
        let mut scene_manager = self.context.scene_manager.try_write().unwrap();
        let scene = scene_manager.get_current_scene_mut().as_mut().unwrap();

        let actions = self.context.editor_actions.clone();

        if let Some(current) = actions.current() {
            let obj = &mut scene[current];

            Label::default()
                .with_text(&obj.name)
                .with_color(Color32::WHITE)
                .render(ctx);

            for (_, comp) in obj.iter_mut() {
                comp.inspector(ctx);
            }
        }
    }
}

impl AttrPanel {
    pub fn new(context: Arc<Context>) -> Self {
        Self { context }
    }
}
