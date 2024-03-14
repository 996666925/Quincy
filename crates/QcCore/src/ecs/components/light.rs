use std::{mem::size_of, sync::Arc};

use log::info;
use nalgebra::{Matrix, Matrix4, Point3, Rotation3, UnitQuaternion, Vector3};
use serde::{Deserialize, Serialize};
use QcMacros::Comp;
use QcRender::{
    buffers::UniformBuffer,
    resources::{Mesh, Shader, Texture},
};

use crate::resources::material::Material;

#[derive(Debug, Comp, Clone, Serialize, Deserialize)]
pub struct Light {
    inner: ComponentInner,
}

impl Light {
    pub fn new() -> Self {
        Self {
            inner: ComponentInner::default(),
        }
    }
}
