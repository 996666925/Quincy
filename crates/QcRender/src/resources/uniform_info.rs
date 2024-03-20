use super::Texture;

use nalgebra::Vector4;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UniformInfo {
    Texture(Texture),
    Vec4(Vector4<f32>),
}
