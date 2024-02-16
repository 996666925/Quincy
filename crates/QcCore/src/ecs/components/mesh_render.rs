use serde::{Deserialize, Serialize};
use QcMacros::Comp;
use QcRender::resources::Model;

#[derive(Debug, Comp, Clone, Serialize, Deserialize)]
pub struct MeshRender {
    models: Vec<Model>,
}

impl MeshRender {
    pub fn new() -> Self {
        Self { models: vec![] }
    }

    pub fn addModel(&mut self, model: Model) {
        self.models.push(model);
    }

    pub fn getModels(&self) -> &Vec<Model> {
        &self.models
    }
    pub fn setModels(&mut self, models: Vec<Model>) {
        self.models = models;
    }
}
