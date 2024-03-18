use std::sync::Arc;

use nalgebra::Matrix4;
use QcCore::{
    ecs::components::{camera::Camera, skybox::SkyBox, transform::Transform},
    resources::material::Material,
};

use super::context::Context;

#[derive(Debug)]
pub struct EditorRenderer {
    context: Arc<Context>,
    material: Material,
}

impl EditorRenderer {
    pub fn new(context: Arc<Context>) -> Self {
        let material = Material::default();
        Self { context, material }
    }

    pub fn render_scene(&self) {
        let mut sceneManager = self.context.scene_manager.try_write().unwrap();
        let mut window = self.context.window.try_read().unwrap();
        let size = window.inner_size().to_logical::<u32>(window.scale_factor());
        let currnetScene = sceneManager
            .getCurrentSceneMut()
            .as_mut()
            .expect("无法获取当前的场景对象");
        if let Some(cameraObj) = currnetScene.get_main_camera() {
            let transform = currnetScene[cameraObj].getComponent::<Transform>().unwrap();

            let mut camera = currnetScene[cameraObj]
                .getComponent::<Camera>()
                .cloned()
                .unwrap();

            let position = transform.position();
            let rotation = transform.rotation();
            camera.cacheMatrices(size.width, size.height, &position, &rotation);
            camera.updateUBO(self.context.engineUBO.clone());

            let local_matrix = transform.get_world_position_matrix(&currnetScene)
                * Matrix4::new_scaling(camera.far / 2f32.sqrt());

            self.context
                .engineUBO
                .setSubData(0, local_matrix.as_slice());

            let renderer = self.context.renderer.try_read().unwrap();

            {
                currnetScene
                    .get_main_skybox()
                    .map(|skybox: thunderdome::Index| {
                        let skybox = currnetScene[skybox].getComponent::<SkyBox>().unwrap();

                        renderer.renderSkybox(skybox, self.context.engineUBO.clone());
                    });
            }

            renderer.renderScene(currnetScene, self.context.engineUBO.clone(), &self.material);
        };
    }
}
