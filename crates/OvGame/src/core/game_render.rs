use log::info;
use nalgebra::{Matrix4, Point3, Rotation3, Vector3};
use std::sync::Arc;
use OvCore::{ecs::components::camera::Camera, resources::material::Material};

use super::context::Context;
use OvCore::ecs::components::transform::Transform;
use OvRender::{
    gl,
    resources::{Mesh, Model},
};
use OvTools::utils::r#ref::Ref;

pub struct GameRender {
    context: Arc<Context>,
    material: Material,
    model: Model,
}

impl GameRender {
    pub fn new(context: Arc<Context>) -> Arc<GameRender> {
        let mut model = Model::new("cube.mesh");
        let material = Material::new("standard");
        Arc::new(Self {
            context,
            model,
            material,
        })
    }

    pub fn renderScene(&self) {
        let mut sceneManager = self.context.sceneManager.try_write().unwrap();

        sceneManager
            .getCurrentSceneMut()
            .as_mut()
            .map(|currnetScene| {
                currnetScene.getMainCamera().map(|cameraObj| {
                    let transform = currnetScene[cameraObj].getComponent::<Transform>().unwrap();
                    let mut camera = currnetScene[cameraObj]
                        .getComponent::<Camera>()
                        .cloned()
                        .unwrap();
                    let position = transform.position();
                    let rotation = transform.rotation();
                    camera.cacheMatrices(&position, &rotation);
                    camera.updateUBO(self.context.engineUBO.clone());
                });

                let renderer = self.context.renderer.try_read().unwrap();
                renderer.setClearColor(0.66, 0.66, 0.66, 1.);
                renderer.clear(true, true, false);
                renderer.renderScene(currnetScene, self.context.engineUBO.clone(), &self.material);
            });
    }
}
