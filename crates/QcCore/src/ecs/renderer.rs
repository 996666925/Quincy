use log::info;
use std::{
    ops::Deref,
    sync::{mpsc::Sender, Arc},
};
use thunderdome::Index;

use nalgebra::Matrix4;
use QcRender::{
    core::{DrawParameters, PrimitiveMode, Renderer as QcRenderer},
    resources::{Mesh, Texture},
    settings::{
        driver_settings::DriverSettings,
        pixel_data::{PixelDataFormat, PixelDataType},
    },
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

#[derive(Debug)]
pub struct Renderer {
    parent: QcRenderer,
    empty_texture: Texture,
    default_material: Material,
}

impl Renderer {
    pub fn new(settings: DriverSettings) -> Ref<Renderer> {
        Ref::new(Self {
            parent: QcRenderer::new(settings),
            empty_texture: Texture::empty(),
            default_material: Material::default(),
        })
    }

    ///临时渲染天空盒，以后必改
    ///temporarily render the skybox ,I must edit this method in future  
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

    pub fn renderScene(&self, scene: &mut Scene, ubo: Arc<MvpUbo>) {
        self.preDraw(Default::default());

        let (drawables, zbuffer_drawables) = self.findAndSortDrawables(scene);

        for drawable in drawables {
            ubo.setSubData(0, drawable.getModelMatrix().as_slice());
            self.drawDrawable(drawable);
        }

        for drawable in zbuffer_drawables {
            self.clear(false, true, false);
            ubo.setSubData(0, drawable.getModelMatrix().as_slice());
            self.drawDrawable(drawable);
        }
    }

    pub fn drawDrawable(&self, drawable: Drawable) {
        self.drawMesh(drawable.getMesh(), drawable.getMaterial());
    }

    pub fn drawMesh(&self, mesh: &Mesh, material: &Material) {
        material.bind(&self.empty_texture);
        self.draw(mesh, PrimitiveMode::TRIANGLES, material.gpu_instances);
    }

    pub fn findAndSortDrawables(&self, scene: &Scene) -> (Drawables, Drawables) {
        self.findAndSortDrawablesWithFunc(scene, |d, _| d)
    }

    pub fn findAndSortDrawablesWithFunc(
        &self,
        scene: &Scene,
        mut func: impl FnMut(Drawable, Index) -> Drawable,
    ) -> (Drawables, Drawables) {
        let mut drawables = Drawables::new();
        let mut zbuffer_drawables = Vec::<(i32, Drawable)>::new();
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
                                    material = &self.default_material;
                                }
                                let mut drawable = Drawable::new(
                                    transform.get_world_matrix(scene),
                                    mesh.clone(),
                                    material.clone(),
                                );

                                if let Some(index) = go.root {
                                    drawable = func(drawable, index);
                                }

                                if let Some(zbuffer) = go.z_buffer {
                                    zbuffer_drawables.push((zbuffer, drawable));
                                } else {
                                    drawables.push(drawable);
                                }
                            }
                        }
                    } else {
                        for model in meshRender.getModels() {
                            for mesh in model.meshes() {
                                drawables.push(Drawable::new(
                                    transform.get_world_matrix(scene),
                                    mesh.clone(),
                                    self.default_material.clone(),
                                ));
                            }
                        }
                    }
                }
            });
        }

        zbuffer_drawables.sort_by(|a, b| a.0.cmp(&b.0));

        let zbuffer_drawables = zbuffer_drawables
            .iter()
            .map(|e| e.1.clone())
            .collect::<Drawables>();
        (drawables, zbuffer_drawables)
    }
}

impl Deref for Renderer {
    type Target = QcRenderer;
    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}
