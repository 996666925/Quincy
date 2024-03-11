use log::info;
use std::{
    ops::Deref,
    sync::{mpsc::Sender, Arc},
};

use nalgebra::Matrix4;
use QcRender::{
    core::{DrawParameters, PrimitiveMode, Renderer as QcRenderer},
    resources::Mesh,
    settings::driver_settings::DriverSettings,
};
use QcTools::utils::r#ref::Ref;

use crate::{resources::material::Material, scene_system::scene::Scene};

use super::{
    components::{
        material_render::MaterialRender, mesh_render::MeshRender, skybox::SkyBox,
        transform::Transform,
    },
    drawable::{Drawable, Drawables},
    MvpUbo,
};

pub struct Renderer {
    parent: QcRenderer,
}

impl Renderer {
    pub fn new(settings: DriverSettings) -> Ref<Renderer> {
        Ref::new(Self {
            parent: QcRenderer::new(settings),
        })
    }

    ///临时渲染天空盒，以后必改
    pub fn renderSkybox(&self, skybox: &SkyBox, ubo: Arc<MvpUbo>) {
        self.preDraw(DrawParameters {
            cull_face: None,
            depth_test: false,
            depth_write: false,
        });

        let drawable = Drawable::new(
            Matrix4::identity(),
            skybox.mesh.clone(),
            skybox.material.clone(),
        );

        self.drawDrawable(drawable);
    }

    pub fn renderScene(&self, scene: &mut Scene, ubo: Arc<MvpUbo>, defaultMaterial: &Material) {
        self.preDraw(Default::default());

        let drawables = self.findAndSortDrawables(scene, defaultMaterial);

        for drawable in drawables {
            ubo.setSubData(0, drawable.getModelMatrix().as_slice());
            self.drawDrawable(drawable);
        }
    }

    pub fn drawDrawable(&self, drawable: Drawable) {
        self.drawMesh(drawable.getMesh(), drawable.getMaterial());
    }

    pub fn drawMesh(&self, mesh: &Mesh, material: &Material) {
        material.bind();

        self.draw(mesh, PrimitiveMode::TRIANGLES, 1);
    }

    pub fn findAndSortDrawables(&self, scene: &Scene, defaultMaterial: &Material) -> Drawables {
        let mut drawables = Drawables::new();
        for (_, go) in scene.iter() {
            if !go.isActive() {
                continue;
            }
            go.getComponent::<Transform>().map(|transform| {
                if let Some(meshRender) = go.getComponent::<MeshRender>() {
                    if let Some(materialRender) = go.getComponent::<MaterialRender>() {
                        let materialList = materialRender.getMaterialList();

                        for model in meshRender.getModels() {
                            for mesh in model.meshes() {
                                let material;
                                if let Some(index) = mesh.getMaterialIndex() {
                                    material = &materialList[index];
                                } else {
                                    material = defaultMaterial;
                                }
                                drawables.push(Drawable::new(
                                    transform.get_world_matrix(scene),
                                    mesh.clone(),
                                    material.clone(),
                                ));
                            }
                        }
                    } else {
                        for model in meshRender.getModels() {
                            for mesh in model.meshes() {
                                drawables.push(Drawable::new(
                                    transform.get_world_matrix(scene),
                                    mesh.clone(),
                                    defaultMaterial.clone(),
                                ));
                            }
                        }
                    }
                }
            });
        }

        drawables
    }
}

impl Deref for Renderer {
    type Target = QcRenderer;
    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}
