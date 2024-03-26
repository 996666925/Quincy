use std::cell::Cell;

use egui::Rgba;
use nalgebra::Vector3;
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
    pub direction: Cell<Option<Direction>>,
    pub operation: Cell<Option<GizmoOperation>>,
    pub target: Cell<Option<Index>>,
    pub first_mouse: Cell<bool>,
}

impl GizmoBehavior {
    pub fn new() -> Self {
        let this = Self {
            direction: Cell::new(None),
            target: Cell::new(None),
            first_mouse: Cell::new(false),
            operation: Cell::new(None),
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
        self.target.get().is_some()
    }

    pub fn start_picking(
        &self,
        target: Index,
        position: Vector3<f32>,
        operation: GizmoOperation,
        direction: Direction,
    ) {
        println!("当前点击的坐标轴：{:?}", direction);
        self.first_mouse.set(true);

        self.operation.set(Some(operation));
        self.direction.set(Some(direction));
    }

    pub fn stop_picking(&self) {}
}
