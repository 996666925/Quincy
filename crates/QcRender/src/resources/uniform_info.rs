use super::Texture;

use nalgebra::{Vector3, Vector4};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UniformInfo {
    Texture(Texture),
    Vec4(Vector4<f32>),
    Vec3(Vector3<f32>),
    I32(i32)
}
