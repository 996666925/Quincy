use std::cell::Cell;

use egui::{Rect, Rgba};
use nalgebra::{Matrix4, Vector2, Vector3};
use thunderdome::Index;
use QcTools::utils::r#ref::Ref;
use QcUI::rect::QcRect;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum GizmoOperation {
    Translate,
    Rotate,
    Scale,
}

impl Into<String> for GizmoOperation {
    fn into(self) -> String {
        match self {
            GizmoOperation::Translate => "Translate",
            GizmoOperation::Rotate => "Rotate",
            GizmoOperation::Scale => "Scale",
        }
        .to_string()
    }
}

impl Into<&str> for GizmoOperation {
    fn into(self) -> &'static str {
        match self {
            GizmoOperation::Translate => "Translate",
            GizmoOperation::Rotate => "Rotate",
            GizmoOperation::Scale => "Scale",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    X = 0,
    Y = 1,
    Z = 2,
}

#[derive(Debug)]
pub struct GizmoBehavior {
    pub direction: Option<Direction>,
    pub operation: Option<GizmoOperation>,
    pub target: Option<Index>,
    pub first_mouse: bool,
    pub origin_mouse: Vector2<f32>,
    pub current_mouse: Vector2<f32>,
}

impl GizmoBehavior {
    pub fn new() -> Self {
        let this = Self {
            direction: None,
            target: None,
            first_mouse: false,
            operation: None,
            origin_mouse: Vector2::identity(),
            current_mouse: Vector2::identity(),
        };
        this
    }

    pub fn get_direction_by_rgba(&self, rgba: &[u8; 4]) -> Option<Direction> {
        if rgba[0] > 250 && rgba[1] == 0 && rgba[2] == 0 {
            Some(Direction::X)
        } else if rgba[1] > 250 && rgba[0] == 0 && rgba[2] == 0 {
            Some(Direction::Y)
        } else if rgba[2] > 250 && rgba[1] == 0 && rgba[0] == 0 {
            Some(Direction::Z)
        } else {
            None
        }
    }

    pub fn is_picking(&self) -> bool {
        self.target.is_some()
    }

    pub fn start_picking(
        &mut self,
        target: Index,
        position: Vector3<f32>,
        operation: GizmoOperation,
        direction: Direction,
    ) {
        println!("当前点击的坐标轴：{:?}", direction);
        self.first_mouse = true;

        self.operation = Some(operation);
        self.direction = Some(direction);
    }

    pub fn stop_picking(&mut self) {}

    pub fn set_current_mouse(&mut self, x: f32, y: f32) {
        if self.first_mouse {
            self.origin_mouse = Vector2::new(x, y);
            self.current_mouse = Vector2::new(x, y);
            self.first_mouse = false;
        } else {
            self.current_mouse = Vector2::new(x, y);
        }
    }

    pub fn apply_operation(
        &mut self,
        view_matrix: Matrix4<f32>,
        proj_matrix: Matrix4<f32>,
        rect: Rect,
    ) {
        if let Some(operation) = self.operation {
            match operation {
                GizmoOperation::Translate => self.apply_translate(view_matrix, proj_matrix, rect),
                GizmoOperation::Rotate => self.apply_rotate(view_matrix, proj_matrix, rect),
                GizmoOperation::Scale => self.apply_scale(view_matrix, proj_matrix, rect),
            }
        }
    }

    fn apply_translate(
        &mut self,
        view_matrix: Matrix4<f32>,
        proj_matrix: Matrix4<f32>,
        rect: Rect,
    ) {
    }

    fn apply_rotate(&mut self, view_matrix: Matrix4<f32>, proj_matrix: Matrix4<f32>, rect: Rect) {}

    fn apply_scale(&mut self, view_matrix: Matrix4<f32>, proj_matrix: Matrix4<f32>, rect: Rect) {}
}
