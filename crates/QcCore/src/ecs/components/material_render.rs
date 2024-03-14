use serde::{Deserialize, Serialize};
use QcMacros::Comp;

use crate::resources::material::Material;

#[derive(Debug, Comp, Clone, Serialize, Deserialize)]
pub struct MaterialRender {
    inner: ComponentInner,
    materialList: Vec<Material>,
}

impl MaterialRender {
    pub fn new() -> Self {
        Self {
            inner: ComponentInner::default(),
            materialList: vec![],
        }
    }

    pub fn addMaterial(&mut self, material: Material) {
        self.materialList.push(material);
    }

    pub fn getMaterialList(&self) -> &Vec<Material> {
        &self.materialList
    }

    pub fn setMaterialList(&mut self, materialList: Vec<Material>) {
        self.materialList = materialList;
    }
}
