use log::info;
use nalgebra::{Matrix4, Point3, Rotation3, Vector3};
use std::sync::Arc;
use QcCore::{
    ecs::components::{camera::Camera, skybox::SkyBox},
    resources::material::Material,
};

use super::context::Context;
use QcCore::ecs::components::transform::Transform;
use QcRender::{
    gl,
    resources::{Mesh, Model},
};
use QcTools::utils::r#ref::Ref;

pub struct GameRender {
    context: Arc<Context>,
    //默认材质
    //default material
    material: Material,
}

impl GameRender {
    pub fn new(context: Arc<Context>) -> Arc<GameRender> {
        let material = Material::default();
        Arc::new(Self { context, material })
    }

    pub fn renderScene(&self) {
        let mut sceneManager = self.context.sceneManager.try_write().unwrap();
        let mut window = self.context.window.try_read().unwrap();
        let size = window.inner_size().to_logical::<u32>(window.scale_factor());
        let currnetScene = sceneManager
            .get_current_scene_mut()
            .as_mut()
            .expect("无法获取当前的场景对象");
        if let Some(cameraObj) = currnetScene.get_main_camera() {
            let transform = currnetScene[cameraObj].getComponent::<Transform>().unwrap();

            let mut camera = currnetScene[cameraObj]
                .getComponent::<Camera>()
                .cloned()
                .unwrap();

            let position = transform.get_world_position(&currnetScene);
            let rotation = transform.rotation();
            camera.cacheMatrices(size.width, size.height, &position.into(), &rotation);
            camera.updateUBO(self.context.engineUBO.clone(), &position);

            let local_matrix = transform.get_world_position_matrix(&currnetScene)
                * Matrix4::new_scaling(camera.far / 2f32.sqrt());

            self.context
                .engineUBO
                .setSubData(0, local_matrix.as_slice());

            let renderer = self.context.renderer.try_read().unwrap();
            renderer.setClearColor(0.66, 0.66, 0.66, 1.);
            renderer.clear(true, true, false);

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
