use std::cell::Cell;

use egui::Rgba;
use nalgebra::{Vector2, Vector3};
use thunderdome::Index;
use QcTools::utils::r#ref::Ref;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum GizmoOperation {
    Translate,
    Rotate,
    Scale,
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
}

impl GizmoBehavior {
    pub fn new() -> Self {
        let this = Self {
            direction: None,
            target: None,
            first_mouse: false,
            operation: None,
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

    pub fn set_current_mouse(&mut self, x: f32, y: f32) {}
}
