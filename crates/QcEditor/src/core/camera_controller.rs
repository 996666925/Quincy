use egui::{Pos2, Response, Vec2};
use nalgebra::{UnitQuaternion, Vector2, Vector3};
use QcCore::ecs::components::camera::Camera;
use QcUI::core::context::UiContext;

#[derive(Debug)]
pub struct CameraController {
    pub camera: Camera,
    pub position: Vector3<f32>,
    pub rotation: Vector3<f32>,
    pub left_mouse_pressed: bool,
    pub middle_mouse_pressed: bool,
    pub right_mouse_pressed: bool,
    pub first_mouse: bool,
    pub last_mouse_pos: Pos2,
    pub mouse_sensitivity: f32,
    pub camera_drag_speed: f32,
}

impl CameraController {
    pub fn new() -> Self {
        let camera = Camera::default().with_fov(60.);

        let position = Vector3::new(0., 3., 1.);
        let rotation = Vector3::new(-45., 0., 0.);

        let left_mouse_pressed = false;
        let middle_mouse_pressed = false;
        let right_mouse_pressed = false;
        let first_mouse = true;
        let last_mouse_pos = Pos2::ZERO;
        let mouse_sensitivity = 0.12f32;
        let camera_drag_speed = 0.03f32;

        Self {
            camera,
            position,
            rotation,
            left_mouse_pressed,
            middle_mouse_pressed,
            right_mouse_pressed,
            first_mouse,
            last_mouse_pos,
            mouse_sensitivity,
            camera_drag_speed,
        }
    }

    pub fn handle_input(&mut self, ctx: &mut UiContext, res: &Response) {
        if res.hovered() {
            self.update_mouse_state_start(ctx.ui);
        }
        self.update_mouse_state_end(ctx.ui);

        // 聚焦物体处理 (TODO)
        {}

        if self.left_mouse_pressed || self.middle_mouse_pressed || self.right_mouse_pressed {
            let pos = ctx.ui.input(|i| i.pointer.interact_pos());

            if let Some(pos) = pos {
                let first_mouse = self.first_mouse;

                if first_mouse {
                    self.last_mouse_pos = pos;
                    self.first_mouse = false;
                }
                let mouse_offset =
                    Pos2::new(pos.x - self.last_mouse_pos.x, self.last_mouse_pos.y - pos.y);
                self.last_mouse_pos = pos;

                if self.right_mouse_pressed {
                    self.handle_camera_fps_mouse(mouse_offset);
                } else {
                    if self.middle_mouse_pressed {
                        self.handle_camera_panning(mouse_offset);
                    }
                }
            }
        }

        if res.hovered() {
            let delta = ctx.ui.input(|i| i.raw_scroll_delta);
            self.handle_camera_zoom(delta);
        }
    }

    pub fn handle_camera_fps_mouse(&mut self, mouse_offset: Pos2) {
        let mouse_offset = mouse_offset * self.mouse_sensitivity;
        self.rotation.x += mouse_offset.y;
        self.rotation.y -= mouse_offset.x;
    }

    pub fn handle_camera_panning(&mut self, mouse_offset: Pos2) {
        let mouse_offset = mouse_offset * self.camera_drag_speed;

        self.position -= self.get_rotation() * Vector3::x() * mouse_offset.x;
        self.position -= self.get_rotation() * Vector3::y() * mouse_offset.y;
    }

    pub fn handle_camera_zoom(&mut self, delta: Vec2) {
        self.position -= self.get_rotation() * Vector3::z() * delta.y / 50.;
    }

    pub fn update_mouse_state_start(&mut self, ui: &mut egui::Ui) {
        if ui.input(|i| i.pointer.primary_pressed()) {
            self.left_mouse_pressed = true;
        }

        if ui.input(|i| i.pointer.button_pressed(egui::PointerButton::Middle)) {
            self.middle_mouse_pressed = true;
        }

        if ui.input(|i| i.pointer.secondary_pressed()) {
            self.right_mouse_pressed = true;
        }
    }

    pub fn update_mouse_state_end(&mut self, ui: &mut egui::Ui) {
        if ui.input(|i| i.pointer.secondary_released()) {
            self.right_mouse_pressed = false;
            self.first_mouse = true;
        }

        if ui.input(|i| i.pointer.primary_released()) {
            self.left_mouse_pressed = false;
            self.first_mouse = true;
        }

        if ui.input(|i| i.pointer.button_released(egui::PointerButton::Middle)) {
            self.middle_mouse_pressed = false;
            self.first_mouse = true;
        }
    }

    pub fn get_rotation(&self) -> UnitQuaternion<f32> {
        UnitQuaternion::from_euler_angles(
            self.rotation.x.to_radians(),
            self.rotation.y.to_radians(),
            self.rotation.z.to_radians(),
        )
    }
}
