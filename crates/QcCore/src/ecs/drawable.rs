use nalgebra::Matrix4;
use QcRender::resources::Mesh;

use crate::resources::material::Material;

pub struct Drawable(Matrix4<f32>, Mesh, Material);
impl Drawable {
    pub fn new(model: Matrix4<f32>, mesh: Mesh, material: Material) -> Self {
        Self(model, mesh, material)
    }

    pub fn getModelMatrix(&self) -> &Matrix4<f32> {
        &self.0
    }

    pub fn getMesh(&self) -> &Mesh {
        &self.1
    }
    pub fn getMaterial(&self) -> &Material {
        &self.2
    }
}

pub type Drawables = Vec<Drawable>;
