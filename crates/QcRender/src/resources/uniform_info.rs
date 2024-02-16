use super::Texture;


use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UniformInfo {
    Texture(Texture),
}
