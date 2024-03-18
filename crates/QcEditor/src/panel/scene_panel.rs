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
pub struct ScenePanel {
    pub context: Arc<Context>,
    pub editor_renderer: Arc<EditorRenderer>,
}

impl DockView for ScenePanel {
    fn render(&mut self, ctx: &mut UiContext, show_tab: bool) {
        let mut rect = ctx.ui.clip_rect();
        if show_tab {
            rect.min.y += 24.;
        }

        let editor_renderer = self.editor_renderer.clone();

        let callback = egui::PaintCallback {
            rect,
            callback: Arc::new(CallbackFn::new(move |info, painter| {
                editor_renderer.render_scene();
            })),
        };
        ctx.ui.painter().add(callback);
    }
}

impl ScenePanel {
    pub fn new(context: Arc<Context>, editor_renderer: Arc<EditorRenderer>) -> Self {
        {
            let mut scene_manager = context.scene_manager.try_write().unwrap();
            let scene = scene_manager.getCurrentSceneMut().as_mut().unwrap();
            let camera = Component::new(Camera::new());
            let skybox = SkyBox::new();

            let transform = Component::new(Transform::new(Point3::new(0., 0., 0.)));
            let mut obj = GameObject::new("Camera");
            obj.insert(camera);
            obj.insert(Component::new(skybox));
            obj.insert(transform);
            scene.add_child(obj);
        }
        Self {
            context,
            editor_renderer,
        }
    }
}
