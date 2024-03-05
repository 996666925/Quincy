use std::{mem::size_of, sync::Arc};

use nalgebra::{Matrix, Matrix4, Point3, Rotation, Rotation3, UnitQuaternion, Vector3};
use QcMacros::Comp;
use QcRender::buffers::UniformBuffer;

use crate::ecs::{component::ComponentInner, MvpUbo};

use serde::{Deserialize, Serialize};

#[derive(Debug, Comp, Clone, Serialize, Deserialize)]
pub struct Transform {
    inner: ComponentInner,

    pub position: Point3<f32>,

    pub rotation: Vector3<f32>,

    pub scale: Vector3<f32>,

    pub local_matrix: Option<Matrix4<f32>>,

    pub world_matrix: Option<Matrix4<f32>>,
}

impl Transform {
    pub fn new(position: Point3<f32>) -> Self {
        Self {
            inner: ComponentInner::default(),
            position,
            rotation: Vector3::zeros(),
            scale: Vector3::new(1., 1., 1.),
            local_matrix: None,
            world_matrix: None,
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
        self.update_local_atrix();
    }

    pub fn setPosition(&mut self, position: Point3<f32>) {
        self.position = position;
        self.update_local_atrix();
    }

    pub fn position(&self) -> Point3<f32> {
        self.position
    }

    pub fn update_local_atrix(&mut self) {
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

        self.local_matrix = Some(modelMatrix);
    }

    pub fn get_local_matrix(&self) -> Matrix4<f32> {
        if let Some(local_matrix) = self.local_matrix {
            return local_matrix;
        }

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

    // pub fn update_world_matrix(&mut self) {
    //     self.
    // }

    pub fn updateUBO(&self, ubo: Arc<MvpUbo>) {}
}
