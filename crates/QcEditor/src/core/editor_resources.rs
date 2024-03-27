use std::collections::HashMap;

use QcRender::resources::Mesh;

use super::gizmo_behavior::GizmoOperation;

#[derive(Debug)]
pub struct EditorResources {
    meshs: HashMap<String, Mesh>,
}

impl EditorResources {
    pub fn new() -> Self {
        let mut meshs = HashMap::new();
        meshs.insert(
            GizmoOperation::Translate.into(),
            Mesh::new("translate.mesh"),
        );
        meshs.insert(GizmoOperation::Rotate.into(), Mesh::new("rotate.mesh"));
        meshs.insert(GizmoOperation::Scale.into(), Mesh::new("scale.mesh"));
        meshs.insert(String::from("plane"), Mesh::new("plane.mesh"));
        Self { meshs }
    }
    pub fn get_mesh(&self, name: &str) -> Option<&Mesh> {
        self.meshs.get(name)
    }
}
