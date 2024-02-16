use std::{mem::size_of, sync::Arc};


use nalgebra::{Matrix, Matrix4, Point3, Rotation, Rotation3, UnitQuaternion, Vector3};
use QcMacros::Comp;
use QcRender::buffers::UniformBuffer;

use crate::ecs::MvpUbo;

use serde::{Deserialize, Serialize};

#[derive(Debug, Comp, Clone, Serialize, Deserialize)]
pub struct Transform {
    position: Point3<f32>,

    rotation: Vector3<f32>,

    scale: Vector3<f32>,

    modelMatrix: Matrix4<f32>,
}

impl Transform {
    pub fn new(position: Point3<f32>) -> Self {
        Self {
            position,
            rotation: Vector3::zeros(),
            scale: Vector3::new(1., 1., 1.),
            modelMatrix: Matrix4::identity(),
        }
    }

    pub fn rotation(&self) -> UnitQuaternion<f32> {
        UnitQuaternion::from_euler_angles(
            self.rotation.x.to_radians(),
            self.rotation.y.to_radians(),
            self.rotation.z.to_radians(),
        )
    }

    pub fn setRotation(&mut self, rotation: Vector3<f32>) {
        self.rotation = rotation;
    }

    pub fn setPosition(&mut self, position: Point3<f32>) {
        self.position = position;
    }
    pub fn position(&self) -> Point3<f32> {
        self.position
    }

    pub fn getModelMatrix(&self) -> Matrix4<f32> {
        let modelMatrix = Matrix4::<f32>::identity();
        let transform = Matrix4::new_translation(&self.position.coords);

        let rotate = UnitQuaternion::from_euler_angles(
            self.rotation.x.to_radians(),
            self.rotation.y.to_radians(),
            self.rotation.z.to_radians(),
        )
        .to_homogeneous();
        let scale = Matrix4::new_nonuniform_scaling(&self.scale);
        let modelMatrix = transform * rotate * scale * modelMatrix;

        modelMatrix
    }

    pub fn updateUBO(&self, ubo: Arc<MvpUbo>) {}
}
