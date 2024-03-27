use std::sync::Arc;

use egui::{
    ahash::{HashMap, HashMapExt},
    Vec2,
};
use nalgebra::{Matrix4, Point3, Vector3, Vector4};
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
use QcUI::rect::QcRect;

use super::{context::Context, gizmo_behavior::GizmoOperation};

#[derive(Debug)]
pub struct EditorRenderer {
    context: Arc<Context>,
    picking_material: Material,
    gizmo_arrow_material: Material,
    grid_material: Material,
}

impl EditorRenderer {
    pub fn new(context: Arc<Context>) -> Self {
        let picking_material = Material::default();

        let gizmo_arrow_material = Material::default()
            .with_shader(Shader::new("gizmo"))
            .with_instances(3);

        let grid_color = Vector3::new(0.176, 0.176, 0.176);
        let mut grid_material = Material::default().with_shader(Shader::new("grid"));
        grid_material.set_uniform_info("uColor", UniformInfo::Vec3(grid_color));

        Self {
            context,
            picking_material,
            gizmo_arrow_material,
            grid_material,
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

            self.render_grid(position);

            renderer.clear(false, true, false);
            renderer.renderScene(currnetScene, self.context.engine_ubo.clone());
        };
    }

    pub fn render_camera(&self) {}

    /// 渲染3D拾取帧缓存
    pub fn render_scene_for_picking(&mut self, rect: &QcRect) {
        let context = self.context.clone();
        let mut scene_manager = context.scene_manager.try_write().unwrap();

        let scene = scene_manager
            .get_current_scene_mut()
            .as_mut()
            .expect("无法获取当前的场景对象");

        if let Some(index) = scene.get_main_camera() {
            let transform = scene[index].getComponent::<Transform>().unwrap();

            let mut camera = scene[index].getComponent::<Camera>().cloned().unwrap();

            let position = transform.get_world_position(&scene);
            let rotation = transform.rotation();
            camera.cacheMatrices(
                rect.width as _,
                rect.height as _,
                &position.into(),
                &rotation,
            );
            camera.updateUBO(self.context.engine_ubo.clone(), &position);

            let local_matrix = transform.get_world_position_matrix(&scene)
                * Matrix4::new_scaling(camera.far / 2f32.sqrt());

            self.context
                .engine_ubo
                .setSubData(0, local_matrix.as_slice());
        }

        let renderer = self.context.renderer.try_read().unwrap();

        let (drawables, zbuffer_drawables) =
            renderer.findAndSortDrawablesWithFunc(scene, |mut d, index| {
                d.set_material(self.picking_material.clone());
                Self::prepare_picking_material(d.get_material_mut(), index);
                d
            });

        let ubo = self.context.engine_ubo.clone();
        for drawable in drawables {
            ubo.setSubData(0, drawable.getModelMatrix().as_slice());
            renderer.drawDrawable(drawable);
        }

        for drawable in zbuffer_drawables {
            renderer.clear(false, true, false);
            ubo.setSubData(0, drawable.getModelMatrix().as_slice());
            renderer.drawDrawable(drawable);
        }
    }

    /// 设置上对应的id
    pub fn prepare_picking_material(picking_material: &mut Material, obj: Index) {
        let [r, g, b, a] = IndexExt::to_rgba_f32(obj);
        let vec4 = Vector4::new(r, g, b, 1.0);

        picking_material.set_uniform_info("uDiffuse", UniformInfo::Vec4(vec4));
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

            let mesh = self
                .context
                .editor_resources
                .get_mesh(operation.into())
                .unwrap();
            let ubo = self.context.engine_ubo.clone();

            ubo.setSubData(0, world_matrix.as_slice());

            self.gizmo_arrow_material.set_uniform_info(
                "uHighlightedAxis",
                UniformInfo::I32(highlighted_gizmo_direction),
            );

            renderer.drawMesh(&mesh, &self.gizmo_arrow_material);
        }
    }

    pub fn render_grid(&self, view_pos: Vector3<f32>) {
        let grid_size = 5000f32;
        let transform = Matrix4::new_translation(&Vector3::new(view_pos.x, 0., view_pos.z));

        let scale =
            Matrix4::new_nonuniform_scaling(&Vector3::new(grid_size * 2., 1., grid_size * 2.));

        let model_matrix = transform * scale;

        let renderer = self.context.renderer.try_read().unwrap();

        renderer.clear(false, true, false);
        let ubo = self.context.engine_ubo.clone();

        ubo.setSubData(0, model_matrix.as_slice());

        let mesh = self.context.editor_resources.get_mesh("plane").unwrap();
        renderer.preDraw(Default::default());
        renderer.drawMesh(mesh, &self.grid_material);
    }
}
