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
use QcMacros::{external, Control};
use QcTools::utils::r#ref::Ref;
use QcUI::{core::context::UiContext, CallbackFn};

use crate::{components::dock::DockView, core::context::Context};

#[derive(Debug)]
pub struct GamePanel {
    pub context: Arc<Context>,
    pub scene: Ref<Scene>,
}

impl DockView for GamePanel {
    fn render(&mut self, ctx: &mut UiContext) {

        let rect = ctx.ui.clip_rect();
        let size = ctx.ui.available_size();
        let context = self.context.clone();
        let scene = self.scene.clone();
        let callback = egui::PaintCallback {
            rect,
            callback: Arc::new(CallbackFn::new(move |info, painter| {
                let ubo = context.engineUBO.clone();

                let renderer = context.renderer.try_read().unwrap();
                renderer.setClearColor(1.0, 0., 0., 1.0);
                renderer.clear(true, false, false);

                let scene = scene.try_write().unwrap();
                if let Some(cameraObj) = scene.get_main_camera() {
                    let transform = scene[cameraObj].getComponent::<Transform>().unwrap();

                    let mut camera = scene[cameraObj].getComponent::<Camera>().cloned().unwrap();

                    let position = transform.position();
                    let rotation = transform.rotation();
                    camera.cacheMatrices(size.x as _, size.y as _, &position, &rotation);
                    camera.updateUBO(context.engineUBO.clone());

                    let local_matrix = transform.get_world_position_matrix(&scene)
                        * Matrix4::new_scaling(camera.far / 2f32.sqrt());

                    context.engineUBO.setSubData(0, local_matrix.as_slice());

                    let renderer = context.renderer.try_read().unwrap();
                    renderer.setClearColor(0.66, 0.66, 0.66, 1.);
                    renderer.clear(true, true, false);
                    // renderer.set_viewport(0, 0, size.x as _, size.y as _);
                    {
                        scene.get_main_skybox().map(|skybox: thunderdome::Index| {
                            let skybox = scene[skybox].getComponent::<SkyBox>().unwrap();

                            renderer.renderSkybox(skybox, context.engineUBO.clone());
                        });
                    }

                    // renderer.renderScene(scene, self.context.engineUBO.clone(), &self.material);
                };

                // rotating_triangle.lock().paint(painter.gl(), angle);
            })),
        };
        ctx.ui.painter().add(callback);
    }
}

impl GamePanel {
    pub fn new(context: Arc<Context>) -> Self {
        let mut scene = Scene::new();

        let camera = Component::new(Camera::new());
        let mut skybox = SkyBox::new();

        let transform = Component::new(Transform::new(Point3::new(0., 0., 0.)));
        let mut obj = GameObject::new("Camera");
        obj.insert(camera);
        obj.insert(Component::new(skybox));
        obj.insert(transform);
        // obj.insert(Component::new(Example));
        scene.add_child(obj);

        Self {
            context,
            scene: Ref::new(scene),
        }
    }
}
