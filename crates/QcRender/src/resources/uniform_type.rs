use super::Texture;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UniformType {
    UNIFORM_BOOL = 0x8B56,
    UNIFORM_INT = 0x1404,
    UNIFORM_FLOAT = 0x1406,
    UNIFORM_FLOAT_VEC2 = 0x8B50,
    UNIFORM_FLOAT_VEC3 = 0x8B51,
    UNIFORM_FLOAT_VEC4 = 0x8B52,
    UNIFORM_FLOAT_MAT4 = 0x8B5C,
    UNIFORM_DOUBLE_MAT4 = 0x8F48,
    UNIFORM_SAMPLER_2D = 0x8B5E,
    UNIFORM_SAMPLER_CUBE = 0x8B60,
}

impl Into<UniformType> for u32 {
    fn into(self) -> UniformType {
        match self {
            0x8B56 => UniformType::UNIFORM_BOOL,
            0x1404 => UniformType::UNIFORM_INT,
            0x1406 => UniformType::UNIFORM_FLOAT,
            0x8B50 => UniformType::UNIFORM_FLOAT_VEC2,
            0x8B51 => UniformType::UNIFORM_FLOAT_VEC3,
            0x8B52 => UniformType::UNIFORM_FLOAT_VEC4,
            0x8B5C => UniformType::UNIFORM_FLOAT_MAT4,
            0x8F48 => UniformType::UNIFORM_DOUBLE_MAT4,
            0x8B5E => UniformType::UNIFORM_SAMPLER_2D,
            0x8B60 => UniformType::UNIFORM_SAMPLER_CUBE,
            _ => panic!("未知的uniform类型"),
        }
    }
}
