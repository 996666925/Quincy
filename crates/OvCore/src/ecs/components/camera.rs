use std::{mem::size_of, sync::Arc};


use log::info;
use nalgebra::{Matrix, Matrix4, Point3, Rotation3, Vector3, UnitQuaternion};
use serde::{Deserialize, Serialize};
use OvMacros::Comp;
use OvRender::buffers::UniformBuffer;
use OvTools::utils::r#ref::Ref;

#[derive(Debug, Comp, Clone, Copy, Serialize, Deserialize)]
pub struct Camera {
    fov: f32,
    near: f32,
    far: f32,
    aspect: f32,

    viewMatrix: Matrix4<f32>,

    projMatrix: Matrix4<f32>,

    viewProjMatrix: Matrix4<f32>,
}

impl Camera {
    pub fn getFov(&self) -> f32 {
        self.fov
    }
    pub fn getNear(&self) -> f32 {
        self.near
    }

    pub fn getFar(&self) -> f32 {
        self.far
    }
    pub fn getAspect(&self) -> f32 {
        self.aspect
    }

    pub fn updateUBO(&self, ubo: Arc<UniformBuffer<[Matrix4<f32>; 3]>>) {
        ubo.setSubData(size_of::<Matrix4<f32>>(), self.viewMatrix.as_slice());
        ubo.setSubData(size_of::<Matrix4<f32>>() * 2, self.projMatrix.as_slice());
    }

    pub fn cacheMatrices(&mut self, position: &Point3<f32>, rotation: &UnitQuaternion<f32>) {
        self.viewMatrix = self.calculateViewMatrix(position, rotation);
        self.projMatrix = self.calculateProjMatrix();
        self.viewProjMatrix = self.projMatrix * self.viewMatrix;
    }

    fn calculateProjMatrix(&self) -> Matrix4<f32> {
        Matrix4::new_perspective(self.aspect, self.fov, self.near, self.far)
    }

    fn calculateViewMatrix(
        &self,
        position: &Point3<f32>,
        rotation: &UnitQuaternion<f32>,
    ) -> Matrix4<f32> {
        let up = rotation * Vector3::y_axis();
        let forward = rotation * Vector3::z();
        let target = position + forward;

        Matrix4::look_at_rh(&target, position, &up)
    }

    pub fn new() -> Self {
        Self {
            fov: 45.,
            near: 0.1,
            far: 1000.,
            aspect: 800. / 600.,
            viewMatrix: Matrix4::zeros(),
            projMatrix: Matrix4::zeros(),
            viewProjMatrix: Matrix4::zeros(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Camera;

    #[test]
    pub fn serde() {
        let camera = Camera::new();
        println!("{:?}", ron::to_string(&camera));
    }

    #[test]
    pub fn deser() {
        let str = "(fov:45.0,near:0.1,far:1000.0,aspect:1.3333334,viewMatrix:(0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0),projMatrix:(0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0),viewProjMatrix:(0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0))";
        let camera: Camera = ron::from_str(str).unwrap();
        println!("{:?}",camera);
    }
}
