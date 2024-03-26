use std::{
    cell::UnsafeCell,
    sync::{Arc, RwLock},
};

use egui::{Key, Pos2, Rect, Vec2};
use nalgebra::{Matrix4, Point3, Vector3};
use serde::{Deserialize, Serialize};
use thunderdome::Index;
use QcCore::{
    ecs::{
        component::Component,
        components::{
            camera::Camera, material_render::MaterialRender, mesh_render::MeshRender,
            skybox::SkyBox, transform::Transform,
        },
        game_object::GameObject,
    },
    resources::material::Material,
};

use QcRender::{
    buffers::{DuckFrameBuffer, FrameBuffer},
    core::DrawParameters,
    gl,
    resources::{Mesh, Texture, TextureKind},
    settings::pixel_data::{PixelDataFormat, PixelDataType},
};

use QcTools::utils::{index_ext::IndexExt, r#ref::Ref, unsafe_box::UnsafeBox};
use QcUI::{core::context::UiContext, rect::QcRect, CallbackFn};

use crate::{
    components::dock::DockView,
    core::{
        context::Context,
        editor_renderer::EditorRenderer,
        gizmo_behavior::{Direction, GizmoOperation},
    },
};

#[derive(Debug)]
pub struct ScenePanel {
    pub context: Arc<Context>,
    pub editor_renderer: Ref<EditorRenderer>,
    picking_framebuffer: DuckFrameBuffer,
    current_opertion: GizmoOperation,
    highlighted_gizmo_direction: Option<Direction>,
}

impl DockView for ScenePanel {
    fn render(&mut self, ctx: &mut UiContext, show_tab: bool) {
        let mut rect = ctx.ui.clip_rect();
        if show_tab {
            rect.min.y += 24.;
        }

        let editor_renderer = self.editor_renderer.clone();
        let axis = if let Some(axis) = self.highlighted_gizmo_direction {
            axis as i32
        } else {
            3
        };

        let callback = egui::PaintCallback {
            rect,
            callback: Arc::new(CallbackFn::new(move |info, painter| {
                let mut editor_renderer = editor_renderer.try_write().unwrap();

                editor_renderer.render_scene(Vec2::new(rect.width(), rect.height()));

                editor_renderer.render_gizmo(GizmoOperation::Translate, axis);
            })),
        };

        ctx.ui.painter().add(callback);

        let res = ctx
            .ui
            .allocate_response(ctx.ui.available_size(), egui::Sense::click());

        // if res.hovered() &&  {
        if res.hovered() {
            self.handle_picking(ctx, rect);
        }

        if res.hovered() && ctx.ui.input(|i| i.pointer.primary_released()) {
            // println!("release")
        }

        // ctx.ui.input(|input|input.key_released(Key::M))
    }
}

impl ScenePanel {
    pub fn new(context: Arc<Context>, editor_renderer: Ref<EditorRenderer>) -> Self {
        {
            let mut scene_manager = context.scene_manager.try_write().unwrap();
            let scene = scene_manager.get_current_scene_mut().as_mut().unwrap();
            let camera = Component::Camera(Camera::new());
            let skybox = SkyBox::new();

            let transform = Component::Transform(Transform::new(Point3::new(0., 0., 3.)));
            let mut obj = GameObject::new("Camera");
            obj.insert(camera);
            obj.insert(Component::SkyBox(skybox));
            obj.insert(transform);

            scene.add_child(obj);

            for i in 0..5 {
                {
                    let mut obj = GameObject::new(&format!("Monkey{}", i));

                    let transform = Transform::new(Point3::new(i as f32 * 2.5 - 5., 0., -3.));

                    let mut meshRender = MeshRender::new();
                    let mut model = Mesh::new("monkey.mesh");
                    model.setMaterialIndex(0);

                    meshRender.addModel(model.into());

                    let mut materialRender = MaterialRender::new();
                    let mut material = Material::default();
                    let image = include_bytes!("../../assets/texture.dds");
                    let texture = Texture::from_bytes(
                        vec![image],
                        TextureKind::Rectangle {
                            width: 0,
                            height: 0,
                        },
                    );
                    material.addTexture(texture);
                    materialRender.addMaterial(material);
                    obj.addComponent(Component::Transform(transform));
                    obj.addComponent(Component::MeshRender(meshRender));
                    obj.addComponent(Component::MaterialRender(materialRender));

                    scene.add_child(obj);
                }
            }
        }

        let picking_framebuffer = DuckFrameBuffer::new();

        Self {
            context,
            editor_renderer,
            picking_framebuffer,
            current_opertion: GizmoOperation::Translate,
            highlighted_gizmo_direction: None,
        }
    }

    pub fn handle_picking(&mut self, ctx: &mut UiContext, rect: Rect) {
        self.render_scene_for_picking(rect);

        let pos = ctx.ui.input(|i| i.pointer.interact_pos());

        if let Some(pos) = pos {
            let window = self.context.window.try_read().unwrap();
            let scale = window.scale_factor();
            let size = window.inner_size();

            let pos = pos * scale as _;
            let mouse_x = pos.x;
            let mouse_y = size.height as f32 - pos.y;

            let renderer = self.context.renderer.try_read().unwrap();
            let mut rgba = [0u8; 4];

            renderer.read_pixels(
                mouse_x as _,
                mouse_y as _,
                1,
                1,
                PixelDataFormat::RGBA,
                PixelDataType::UNSIGNED_BYTE,
                rgba.as_ptr() as _,
            );

            rgba[3] = 0;

            let gizmo = self.context.gizmo_behavior.clone();

            let direction = if gizmo.is_picking() {
                gizmo.direction.get()
            } else {
                gizmo.get_direction_by_rgba(&rgba)
            };

            let context = self.context.clone();
            let mut scene_manager = context.scene_manager.try_write().unwrap();

            let scene = scene_manager
                .get_current_scene_mut()
                .as_mut()
                .expect("无法获取当前的场景对象");

            self.highlighted_gizmo_direction = direction;

            // 如果触发点击，进行选中处理
            if ctx.ui.input(|i| i.pointer.primary_pressed()) {
                if let Some(direction) = direction {
                    let actions = self.context.editor_actions.clone();

                    gizmo.start_picking(
                        actions.target.get().unwrap(),
                        Vector3::identity(),
                        self.current_opertion,
                        direction,
                    );
                } else {
                    let id = IndexExt::u8_to_index(rgba);

                    // 设置选中对象
                    let actions = self.context.editor_actions.clone();
                    if let Some((index, obj)) = &scene.get_by_slot(id.slot) {
                        actions.select(Some(*index));
                    } else {
                        actions.select(None);
                    }
                }
            }

            // gizmo的拖拽处理
            if gizmo.is_picking() {}
        }
    }

    pub fn render_scene_for_picking(&self, rect: Rect) {
        let window = self.context.window.try_read().unwrap();
        let scale = window.scale_factor();
        let size = window.inner_size();
        let gl_rect = QcRect::to_gl_rect(rect, size, scale as _);

        self.picking_framebuffer.resize(
            gl_rect.x as _,
            gl_rect.y as _,
            gl_rect.width as _,
            gl_rect.height as _,
        );

        let renderer = self.context.renderer.try_read().unwrap();
        renderer.setClearColor(1.0, 1.0, 1.0, 1.0);
        renderer.clear(true, true, true);
        renderer.preDraw(DrawParameters {
            depth_test: true,
            ..Default::default()
        });
        let mut editor_renderer = self.editor_renderer.try_write().unwrap();

        editor_renderer.render_scene_for_picking();

        let editor_actions = self.context.editor_actions.clone();

        // 如果游戏对象被选中
        if let Some(target) = editor_actions.target.get() {
            editor_renderer.render_gizmo(GizmoOperation::Translate, 666);
        }
    }
}
