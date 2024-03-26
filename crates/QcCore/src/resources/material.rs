use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use QcRender::resources::{Shader, Texture, UniformInfo};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Material {
    shader: Shader,
    uniformInfoList: HashMap<String, UniformInfo>,
    pub gpu_instances: u32,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            shader: Default::default(),
            uniformInfoList: Default::default(),
            gpu_instances: 1,
        }
    }
}

impl Material {
    pub fn with_shader(mut self, shader: Shader) -> Self {
        self.shader = shader;
        self
    }

    pub fn with_instances(mut self, instances: u32) -> Self {
        self.gpu_instances = instances;
        self
    }

    pub fn bind(&self, empty_texture: &Texture) {
        self.shader.bind();

        let mut slot = 0;
        for (name, uniformInfo) in self.uniformInfoList.iter() {
            match uniformInfo {
                UniformInfo::Texture(texture) => {
                    texture.bind(slot);
                    slot += 1;
                }
                UniformInfo::Vec4(vec4) => {
                    self.shader.set_uniform_vec4(name, vec4);
                }
                UniformInfo::I32(i32) => {
                    self.shader.set_uniform_i32(name, *i32);
                }
            }
        }

        if slot == 0 {
            empty_texture.bind(0);
        }
    }

    pub fn addTexture(&mut self, texture: Texture) {
        self.uniformInfoList
            .insert("Texture".to_string(), UniformInfo::Texture(texture));
    }

    pub fn set_uniform_info(&mut self, name: &str, info: UniformInfo) {
        self.uniformInfoList
            .entry(name.to_string())
            .and_modify(|key| *key = info.clone())
            .or_insert(info);
    }

    pub fn getUniformList(&self) -> &HashMap<String, UniformInfo> {
        &self.uniformInfoList
    }
    pub fn setUniformList(&mut self, uniformList: HashMap<String, UniformInfo>) {
        self.uniformInfoList = uniformList;
    }
}
