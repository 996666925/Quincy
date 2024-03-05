use std::{
    ops::Deref,
    sync::{mpsc::Sender, Arc},
};
use log::info;

use nalgebra::Matrix4;
use QcRender::{
    core::{PrimitiveMode, Renderer as QcRenderer},
    resources::Mesh,
    settings::driver_settings::DriverSettings,
};
use QcTools::utils::r#ref::Ref;

use crate::{resources::material::Material, scene_system::scene::Scene};

use super::{
    components::{material_render::MaterialRender, mesh_render::MeshRender, transform::Transform},
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

    pub fn renderScene(&self, scene: &mut Scene, ubo: Arc<MvpUbo>, defaultMaterial: &Material) {
        self.preDraw();

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
                                    transform.get_local_matrix(),
                                    mesh.clone(),
                                    material.clone(),
                                ));
                            }
                        }
                    } else {
                        for model in meshRender.getModels() {
                            for mesh in model.meshes() {
                                drawables.push(Drawable::new(
                                    transform.get_local_matrix(),
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
