use nalgebra::{Point3, Vector2, Vector3};
use serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Clone, Copy, Serialize, Deserialize)]
pub struct Vertex {
    position: Point3<f32>,
    texCoords: Vector2<f32>,
    normals: Vector3<f32>,
}

impl Vertex {
    pub fn new(position: Point3<f32>, texCoords: Vector2<f32>, normals: Vector3<f32>) -> Self {
        Self {
            position,
            texCoords,
            normals,
        }
    }

    pub fn from_position(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: Point3::new(x, y, z),
            texCoords: Vector2::identity(),
            normals: Vector3::identity(),
        }
    }
}
