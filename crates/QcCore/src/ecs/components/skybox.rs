use std::{mem::size_of, sync::Arc};

use log::info;
use nalgebra::{Matrix, Matrix4, Point3, Rotation3, UnitQuaternion, Vector3};
use serde::{Deserialize, Serialize};
use QcMacros::Comp;
use QcRender::{
    buffers::UniformBuffer,
    resources::{Mesh, Shader, Texture},
};

use crate::{ecs::component::ComponentInner, resources::material::Material};

#[derive(Debug, Comp, Clone, Serialize, Deserialize)]
pub struct SkyBox {
    inner: ComponentInner,
    pub mesh: Mesh,
    pub material: Material,
}

impl SkyBox {
    pub fn new() -> Self {
        let mut material = Material::default().with_shader(Shader::skybox());
        material.addTexture(Texture::skybox());

        Self {
            inner: ComponentInner::default(),
            mesh: Mesh::cube(),
            material,
        }
    }
}
