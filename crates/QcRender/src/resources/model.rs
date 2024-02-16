
use serde::{Deserialize, Serialize};

use crate::resources::mesh::Mesh;
use crate::Asset;
use std::ops::{Deref, DerefMut, Index};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Model {
    name: String,
    meshes: Vec<Mesh>,
}

impl Index<usize> for Model {
    type Output = Mesh;

    fn index(&self, index: usize) -> &Self::Output {
        &self.meshes[index]
    }
}

impl Deref for Model {
    type Target = Vec<Mesh>;

    fn deref(&self) -> &Self::Target {
        &self.meshes
    }
}
impl DerefMut for Model {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.meshes
    }
}

impl Model {
    pub fn meshes(&self) -> &Vec<Mesh> {
        &self.meshes
    }

    pub fn new(name: &str) -> Model {
        Self {
            name: name.to_string(),
            meshes: vec![],
        }
    }

    pub fn addMesh(&mut self, mesh: Mesh) {
        self.meshes.push(mesh);
    }
}

impl From<Mesh> for Model {
    fn from(value: Mesh) -> Self {
        Self {
            name: "Model".to_string(),
            meshes: vec![value],
        }
    }
}
