use std::sync::{Arc, RwLock};

use egui::{Pos2, Rect, Vec2};
use nalgebra::{Matrix4, Point3};
use serde::{Deserialize, Serialize};
use QcCore::{
    ecs::{
        component::Component,
        components::{camera::Camera, skybox::SkyBox, transform::Transform},
        game_object::GameObject,
    },
    scene_system::scene::Scene,
};

use QcTools::utils::r#ref::Ref;
use QcUI::{core::context::UiContext, CallbackFn};

use crate::{
    components::dock::DockView,
    core::{context::Context, editor_renderer::EditorRenderer},
};

#[derive(Debug)]
pub struct GamePanel {
    pub context: Arc<Context>,
    pub editor_renderer: Ref<EditorRenderer>,
}

impl DockView for GamePanel {
    fn render(&mut self, ctx: &mut UiContext, show_tab: bool) {
        let mut rect = ctx.ui.clip_rect();
        if show_tab {
            rect.min.y += 24.;
        }

        let editor_renderer = self.editor_renderer.clone();

        let callback = egui::PaintCallback {
            rect,
            callback: Arc::new(CallbackFn::new(move |info, painter| {
                let editor_renderer = editor_renderer.try_read().unwrap();

                editor_renderer.render_scene(Vec2::new(rect.width(), rect.height()));
            })),
        };
        ctx.ui.painter().add(callback);
    }
}

impl GamePanel {
    pub fn new(context: Arc<Context>, editor_renderer: Ref<EditorRenderer>) -> Self {
         
        Self {
            context,
            editor_renderer,
        }
    }
}
