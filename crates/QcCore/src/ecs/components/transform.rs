use std::{cell::Cell, mem::size_of, sync::Arc};

use nalgebra::{Matrix, Matrix4, Point3, Rotation, Rotation3, UnitQuaternion, Vector3};
use QcMacros::Comp;
use QcRender::buffers::UniformBuffer;

use crate::ecs::{component::ComponentInner, graph::Graph, MvpUbo};

use serde::{Deserialize, Serialize};

#[derive(Debug, Comp, Clone, Serialize, Deserialize)]
pub struct Transform {
    inner: ComponentInner,

    pub position: Point3<f32>,

    pub rotation: Vector3<f32>,

    pub scale: Vector3<f32>,

    pub local_matrix: Cell<Matrix4<f32>>,

    pub world_matrix: Cell<Matrix4<f32>>,

    pub dirty: Cell<bool>,
}

impl Transform {
    pub fn new(position: Point3<f32>) -> Self {
        Self {
            inner: ComponentInner::default(),
            position,
            rotation: Vector3::zeros(),
            scale: Vector3::new(1., 1., 1.),
            local_matrix: Cell::new(Matrix4::identity()),
            world_matrix: Cell::new(Matrix4::identity()),
            dirty: Cell::new(true),
        }
    }

    pub fn rotation(&self) -> UnitQuaternion<f32> {
        UnitQuaternion::from_euler_angles(
            self.rotation.x.to_radians(),
            self.rotation.y.to_radians(),
            self.rotation.z.to_radians(),
        )
    }

    pub fn set_scale(&mut self, scale: Vector3<f32>) {
        if self.dirty.get() || self.scale != scale {
            self.scale = scale;
            self.dirty.set(true);
        }
    }

    pub fn set_rotation(&mut self, rotation: Vector3<f32>) {
        if self.dirty.get() || self.rotation != rotation {
            self.rotation = rotation;
            self.dirty.set(true);
        }
    }

    pub fn set_position(&mut self, position: Point3<f32>) {
        if self.dirty.get() || self.position != position {
            self.position = position;
            self.dirty.set(true);
        }
    }

    pub fn position(&self) -> Point3<f32> {
        self.position
    }

    pub fn update_local_atrix(&self) {
        if self.dirty.take() {
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

            self.local_matrix.set(modelMatrix);
        }
    }

    pub fn get_local_matrix(&self) -> Matrix4<f32> {
        self.local_matrix.get()
    }

    pub fn get_world_matrix(&self, graph: &Graph) -> Matrix4<f32> {
        self.update_local_atrix();

        let mut world_matrix = self.local_matrix.get();
        println!("{:?}", self);
        if let Some(parent) = self.parent {
            let mut parentIdx = graph[parent].parent;
            println!("id:{:?},parent:{:?}", parent, parentIdx);
            while let Some(parent) = parentIdx {
                let obj = &graph[parent];
                if let Some(transform) = obj.getComponent::<Transform>() {
                    transform.update_local_atrix();

                    world_matrix = world_matrix * transform.local_matrix.get();
                }

                parentIdx = obj.parent;
            }
        }

        world_matrix
    }

    pub fn updateUBO(&self, ubo: Arc<MvpUbo>) {}
}
