use serde::{Deserialize, Serialize};
use QcRender::resources::{Shader, Texture, UniformInfo};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Material {
    shader: Shader,
    uniformInfoList: Vec<UniformInfo>,
}

impl Material {
    pub fn with_shader(mut self, shader: Shader) -> Self {
        self.shader = shader;
        self
    }

    pub fn bind(&self) {
        self.shader.bind();

        let mut slot = 0;
        for uniformInfo in self.uniformInfoList.iter() {
            match uniformInfo {
                UniformInfo::Texture(texture) => {
                    texture.bind(slot);
                    slot += 1;
                }
            }
        }
    }

    pub fn addTexture(&mut self, texture: Texture) {
        self.uniformInfoList.push(UniformInfo::Texture(texture));
    }

    pub fn getUniformList(&self) -> &Vec<UniformInfo> {
        &self.uniformInfoList
    }
    pub fn setUniformList(&mut self, uniformList: Vec<UniformInfo>) {
        self.uniformInfoList = uniformList;
    }
}
