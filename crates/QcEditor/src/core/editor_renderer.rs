use std::sync::Arc;

use egui::Vec2;
use nalgebra::{Matrix4, Vector4};
use thunderdome::{Arena, Index};
use QcCore::{
    ecs::{
        components::{
            camera::Camera, material_render::MaterialRender, mesh_render::MeshRender,
            skybox::SkyBox, transform::Transform,
        },
        drawable::{Drawable, Drawables},
    },
    resources::material::Material,
};
use QcRender::resources::UniformInfo;
use QcTools::utils::index_ext::IndexExt;

use super::context::Context;

#[derive(Debug)]
pub struct EditorRenderer {
    context: Arc<Context>,
    material: Material,
    picking_material: Material,
}

impl EditorRenderer {
    pub fn new(context: Arc<Context>) -> Self {
        let material = Material::default();

        let picking_material = Material::default();
        // picking_material
        Self {
            context,
            material,
            picking_material,
        }
    }

    /// 渲染游戏界面
    pub fn render_scene(&self, size: Vec2) {
        let mut sceneManager = self.context.scene_manager.try_write().unwrap();
        let mut window = self.context.window.try_read().unwrap();
        let size = size * window.scale_factor() as f32;
        let currnetScene = sceneManager
            .get_current_scene_mut()
            .as_mut()
            .expect("无法获取当前的场景对象");
        if let Some(index) = currnetScene.get_main_camera() {
            let transform = currnetScene[index].getComponent::<Transform>().unwrap();

            let mut camera = currnetScene[index]
                .getComponent::<Camera>()
                .cloned()
                .unwrap();

            let position = transform.position();
            let rotation = transform.rotation();
            camera.cacheMatrices(size.x as _, size.y as _, &position, &rotation);
            camera.updateUBO(self.context.engine_ubo.clone());

            let local_matrix = transform.get_world_position_matrix(&currnetScene)
                * Matrix4::new_scaling(camera.far / 2f32.sqrt());

            self.context
                .engine_ubo
                .setSubData(0, local_matrix.as_slice());

            let renderer = self.context.renderer.try_read().unwrap();

            {
                currnetScene
                    .get_main_skybox()
                    .map(|skybox: thunderdome::Index| {
                        let skybox = currnetScene[skybox].getComponent::<SkyBox>().unwrap();

                        renderer.renderSkybox(skybox, self.context.engine_ubo.clone());
                    });
            }

            renderer.renderScene(
                currnetScene,
                self.context.engine_ubo.clone(),
                &self.material,
            );
        };
    }

    pub fn render_camera(&self) {}

    /// 渲染3D拾取帧缓存
    pub fn render_scene_for_picking(&mut self) {
        let context = self.context.clone();
        let mut scene_manager = context.scene_manager.try_write().unwrap();

        let scene = scene_manager
            .get_current_scene_mut()
            .as_mut()
            .expect("无法获取当前的场景对象");

        let mut window = context.window.try_read().unwrap();
        let size = window.inner_size().to_logical::<u32>(window.scale_factor());

        if let Some(index) = scene.get_main_camera() {
            let transform = scene[index].getComponent::<Transform>().unwrap();

            let mut camera = scene[index].getComponent::<Camera>().cloned().unwrap();

            let position = transform.position();
            let rotation = transform.rotation();
            camera.cacheMatrices(size.width, size.height, &position, &rotation);
            camera.updateUBO(self.context.engine_ubo.clone());

            let local_matrix = transform.get_world_position_matrix(&scene)
                * Matrix4::new_scaling(camera.far / 2f32.sqrt());

            self.context
                .engine_ubo
                .setSubData(0, local_matrix.as_slice());
        }

        let mut drawables = Drawables::new();
        for (index, go) in scene.iter() {
            if !go.isActive() {
                continue;
            }
            if let Some(transform) = go.getComponent::<Transform>() {
                let world_matrix = transform.get_world_matrix(&scene);

                if let Some(mesh_render) = go.getComponent::<MeshRender>() {
                    self.prepare_picking_material(index);

                    if let Some(material_render) = go.getComponent::<MaterialRender>() {
                        for model in mesh_render.getModels() {
                            for mesh in model.meshes() {
                                drawables.push(Drawable::new(
                                    world_matrix,
                                    mesh.clone(),
                                    self.picking_material.clone(),
                                ));
                            }
                        }
                    }
                }
            }
        }

        let ubo = self.context.engine_ubo.clone();
        let renderer = self.context.renderer.try_read().unwrap();

        for drawable in drawables {
            ubo.setSubData(0, drawable.getModelMatrix().as_slice());
            renderer.drawDrawable(drawable);
        }
    }

    /// 设置上对应的id
    pub fn prepare_picking_material(&mut self, obj: Index) {
        let [r, g, b, a] = IndexExt::to_rgba_f32(obj);
        let vec4 = Vector4::new(r, g, b, 1.0);

        self.picking_material
            .set_uniform_info("uDiffuse", UniformInfo::Vec4(vec4));
    }
}
