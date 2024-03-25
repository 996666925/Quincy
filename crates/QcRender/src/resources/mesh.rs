use crate::buffers::{IndexBuffer, Type, VertexArray, VertexBuffer};
use crate::geometry::Vertex;
use crate::Asset;
use ron::error::SpannedError;
use rust_embed::EmbeddedFile;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
use std::mem::size_of;
use QcTools::sync::{Lazy, OnceCell};

#[derive(Serialize, Deserialize)]
pub struct MeshFile {
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
}

impl MeshFile {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u16>) -> Self {
        Self { vertices, indices }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mesh {
    name: String,
    vertexCount: i32,
    indexCount: i32,
    vertexArray: VertexArray,
    vertexBuffer: VertexBuffer,
    indexBuffer: IndexBuffer,
    materialIndex: Option<usize>,
}

impl Mesh {
    pub fn cube() -> Self {
        Mesh::new("skybox.mesh")
    }

    pub fn plane() -> Self {
        Mesh::new("plane.mesh")
    }

    pub fn new(name: &str) -> Self {
        let file =
            Asset::get(&format!("model/{}", name)).expect(&format!("找不到资源文件:{}", name));

        let data =
            String::from_utf8(file.data.to_vec()).expect(&format!("读取Mesh文件({})失败", name));
        let mut mesh = Mesh::loadByMeshFile(&data).expect(&format!("加载Mesh文件({})失败", name));
        mesh.name = name.to_string();
        mesh
    }

    pub fn create(vertices: Vec<Vertex>, indices: Vec<u16>) -> Self {
        let vertexCount = vertices.len() as _;
        let indexCount = indices.len() as _;
        let (vertexArray, vertexBuffer, indexBuffer) = Self::createBuffers(vertices, indices);
        Self {
            name: "name".to_string(),
            vertexCount,
            indexCount,
            vertexArray,
            vertexBuffer,
            indexBuffer,
            materialIndex: None,
        }
    }
    pub fn getMaterialIndex(&self) -> Option<usize> {
        self.materialIndex
    }
    pub fn setMaterialIndex(&mut self, index: usize) {
        self.materialIndex = Some(index);
    }

    pub fn getName(&self) -> &str {
        &self.name
    }
    pub fn bind(&self) {
        self.vertexArray.bind();
    }

    pub fn unbind(&self) {
        self.vertexArray.unbind();
    }

    pub fn getVertexCount(&self) -> i32 {
        self.vertexCount
    }

    pub fn getIndexCount(&self) -> i32 {
        self.indexCount
    }

    pub fn createBuffers(
        vertices: Vec<Vertex>,
        indices: Vec<u16>,
    ) -> (VertexArray, VertexBuffer, IndexBuffer) {
        let vertexArray = VertexArray::new();
        let vertexBuffer = VertexBuffer::new(vertices.as_slice());
        let indexBuffer = IndexBuffer::new(indices.as_slice());
        vertexArray.bind();
        vertexBuffer.bind();
        indexBuffer.bind();
        vertexArray.bindAttribute(0, 3, Type::FLOAT, size_of::<Vertex>(), 0);
        vertexArray.bindAttribute(1, 2, Type::FLOAT, size_of::<Vertex>(), size_of::<f32>() * 3);
        vertexArray.bindAttribute(2, 3, Type::FLOAT, size_of::<Vertex>(), size_of::<f32>() * 5);
        (vertexArray, vertexBuffer, indexBuffer)
    }

    pub fn loadByMeshFile(data: &str) -> Result<Mesh, SpannedError> {
        let meshFile: MeshFile = ron::from_str(data)?;
        Ok(Self::create(meshFile.vertices, meshFile.indices))
    }
}
