use std::{
    cell::UnsafeCell,
    sync::{Arc, RwLock},
};

use egui::{Key, Pos2, Rect, Vec2};
use nalgebra::{Matrix4, Point3};
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
    buffers::FrameBuffer,
    gl,
    resources::{Mesh, Texture, TextureKind},
    settings::pixel_data::{PixelDataFormat, PixelDataType},
};

use QcTools::utils::{index_ext::IndexExt, r#ref::Ref, unsafe_box::UnsafeBox};
use QcUI::{core::context::UiContext, CallbackFn};

use crate::{
    components::dock::DockView,
    core::{context::Context, editor_renderer::EditorRenderer},
};

#[derive(Debug)]
pub struct ScenePanel {
    pub context: Arc<Context>,
    pub editor_renderer: Ref<EditorRenderer>,
    picking_framebuffer: FrameBuffer,
}

impl DockView for ScenePanel {
    fn render(&mut self, ctx: &mut UiContext, show_tab: bool) {
        let mut rect = ctx.ui.clip_rect();
        if show_tab {
            rect.min.y += 24.;
        }

        let editor_renderer = self.editor_renderer.clone();

        let callback = egui::PaintCallback {
            rect,
            callback: Arc::new(CallbackFn::new(move |info, painter| {
                let editor_renderer = editor_renderer.try_read().unwrap();

                editor_renderer.render_scene();
            })),
        };

        ctx.ui.painter().add(callback);

        let res = ctx
            .ui
            .allocate_response(ctx.ui.available_size(), egui::Sense::click());

        if res.hovered() && ctx.ui.input(|i| i.pointer.primary_pressed()) {
        // if res.hovered() {
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
            let camera = Component::new(Camera::new());
            let skybox = SkyBox::new();

            let transform = Component::new(Transform::new(Point3::new(0., 0., 3.)));
            let mut obj = GameObject::new("Camera");
            obj.insert(camera);
            obj.insert(Component::new(skybox));
            obj.insert(transform);

            scene.add_child(obj);

            let obj = {
                let mut obj = GameObject::new("Monkey");

                let transform = Transform::new(Point3::new(0., 0., -3.));

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
                obj.addComponent(Component::new(transform));
                obj.addComponent(Component::new(meshRender));
                obj.addComponent(Component::new(materialRender));
                obj
            };

            let index = scene.add_child(obj);
        }

        let picking_framebuffer = FrameBuffer::new(800, 600);
        Self {
            context,
            editor_renderer,
            picking_framebuffer,
        }
    }

    pub fn handle_picking(&self, ctx: &mut UiContext, rect: Rect) {
        self.render_scene_for_picking(rect);

        let pos = ctx.ui.input(|i| i.pointer.interact_pos());

        if let Some(pos) = pos {
            let window = self.context.window.try_read().unwrap();
            let scale = window.scale_factor();
            let size = window.inner_size();

            let pos = pos * scale as _;
            let mouse_x = pos.x;
            let mouse_y = size.height as f32 - pos.y;

            // self.picking_framebuffer.bind();

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
            // println!("{:?}", rgba);

            rgba[3] = 0;

            let id = IndexExt::u8_to_index(rgba);

            let context = self.context.clone();
            let mut scene_manager = context.scene_manager.try_write().unwrap();

            let scene = scene_manager
                .get_current_scene_mut()
                .as_mut()
                .expect("无法获取当前的场景对象");

            if let Some((_, obj)) = &scene.get_by_slot(id.slot) {
                println!("当前点击的是:{:?}", obj.name);
            } else {
                println!("什么也没点中");
            }

            self.picking_framebuffer.unbind();
        }
    }

    pub fn render_scene_for_picking(&self, rect: Rect) {
        unsafe {
            let window = self.context.window.try_read().unwrap();
            let scale = window.scale_factor();
            let size = window.inner_size();
            gl::Enable(gl::SCISSOR_TEST);

            let rect = rect * scale as _;
            gl::Scissor(
                rect.min.x as _,
                (size.height - rect.max.y as u32) as _,
                rect.width() as _,
                rect.height() as _,
            );
            gl::Viewport(
                rect.min.x as _,
                (size.height - rect.max.y as u32) as _,
                rect.width() as _,
                rect.height() as _,
            );
        }

        // self.picking_framebuffer
        //     .resize(rect.width() as _, rect.height() as _);

        // self.picking_framebuffer.bind();

        let renderer = self.context.renderer.try_read().unwrap();
        renderer.setClearColor(1.0, 1.0, 1.0, 1.0);
        renderer.clear(true, true, true);

        let mut editor_renderer = self.editor_renderer.try_write().unwrap();

        editor_renderer.render_scene_for_picking();

        self.picking_framebuffer.unbind();
    }
}
