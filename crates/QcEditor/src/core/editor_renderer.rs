use std::sync::Arc;

use egui::{
    ahash::{HashMap, HashMapExt},
    Vec2,
};
use nalgebra::{Matrix4, Point3, Vector4};
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
use QcRender::{
    core::DrawParameters,
    resources::{Mesh, Shader, UniformInfo},
};
use QcTools::utils::index_ext::IndexExt;

use super::{context::Context, gizmo_behavior::GizmoOperation};

#[derive(Debug)]
pub struct EditorRenderer {
    context: Arc<Context>,
    material: Material,
    picking_material: Material,
    gizmo_arrow_material: Material,
    gizmo_meshs: HashMap<GizmoOperation, Mesh>,
}

impl EditorRenderer {
    pub fn new(context: Arc<Context>) -> Self {
        let material = Material::default();

        let picking_material = Material::default();
        let gizmo_arrow_material = Material::default()
            .with_shader(Shader::new("gizmo"))
            .with_instances(3);

        let mut gizmo_meshs = HashMap::new();
        gizmo_meshs.insert(GizmoOperation::Translate, Mesh::new("translate.mesh"));
        gizmo_meshs.insert(GizmoOperation::Rotate, Mesh::new("rotate.mesh"));
        gizmo_meshs.insert(GizmoOperation::Scale, Mesh::new("scale.mesh"));

        Self {
            context,
            material,
            picking_material,
            gizmo_arrow_material,
            gizmo_meshs,
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

            let position = transform.get_world_position(&currnetScene);
            let rotation = transform.rotation();
            camera.cacheMatrices(size.x as _, size.y as _, &position.into(), &rotation);
            camera.updateUBO(self.context.engine_ubo.clone(), &position);

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

            renderer.clear(false, true, false);
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

            let position = transform.get_world_position(&scene);
            let rotation = transform.rotation();
            camera.cacheMatrices(size.width, size.height, &position.into(), &rotation);
            camera.updateUBO(self.context.engine_ubo.clone(), &position);

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

    pub fn render_gizmo(&mut self, operation: GizmoOperation, highlighted_gizmo_direction: i32) {
        let editor_actions = self.context.editor_actions.clone();

        if let Some(target) = editor_actions.target.get() {
            let mut scene_manager = self.context.scene_manager.try_write().unwrap();

            let scene = scene_manager
                .get_current_scene_mut()
                .as_mut()
                .expect("无法获取当前的场景对象");

            let current = &scene[target];

            let transform = current.getComponent::<Transform>().unwrap();

            let world_matrix = transform.get_world_position_matrix(&scene);

            let renderer = self.context.renderer.try_read().unwrap();

            renderer.clear(false, true, true);

            let mesh = self.gizmo_meshs.get(&operation).unwrap();
            let ubo = self.context.engine_ubo.clone();

            ubo.setSubData(0, world_matrix.as_slice());

            self.gizmo_arrow_material
                .set_uniform_info("uHighlightedAxis", UniformInfo::I32(highlighted_gizmo_direction));
            
            renderer.drawMesh(&mesh, &self.gizmo_arrow_material);
        }
    }
}
