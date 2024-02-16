use crate::Asset;

use rust_embed::EmbeddedFile;
use std::collections::HashMap;
use std::ffi::CString;
use std::fmt::format;
use std::ptr;
use std::sync::Mutex;
use QcTools::sync::Lazy;
use serde::{Serialize, Deserialize};

static SHADER_MAP: Lazy<Mutex<Vec<Shader>>> = Lazy::new(|| Mutex::new(Vec::new()));


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shader {
    name: String,
    program: u32,
}

impl Shader {
    pub fn new(name: &str) -> Self {
        //先查询现有的ShaderMap
        let mut map = SHADER_MAP.try_lock().unwrap();
        if let Some(shader) = map.iter().find(|shader| shader.name == name) {
            return shader.clone();
        }
        //查不到再创建Shader
        let (vertex, fragment) = Shader::findShader(name);
        let program = Shader::createProgram(vertex, fragment);
        let shader = Self {
            name: name.to_string(),
            program,
        };
        map.push(shader.clone());
        shader
    }

    pub fn findShader(name: &str) -> (EmbeddedFile, EmbeddedFile) {
        let vertexShader = Asset::get(&format!("shader/output/{name}.vert.spv")).unwrap();
        let fragmentShader = Asset::get(&format!("shader/output/{name}.frag.spv")).unwrap();

        (vertexShader, fragmentShader)
    }

    pub fn createProgram(vertexSource: EmbeddedFile, fragmentSource: EmbeddedFile) -> u32 {
        unsafe {
            let shaderSource = vertexSource.data;
            let vertexShader = gl::CreateShader(gl::VERTEX_SHADER);
            let fragmentShader = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderBinary(
                1,
                &vertexShader,
                gl::SHADER_BINARY_FORMAT_SPIR_V,
                shaderSource.as_ptr() as _,
                shaderSource.len() as _,
            );
            let entry = CString::new("main").unwrap();
            gl::SpecializeShader(
                vertexShader,
                entry.as_ptr() as _,
                0,
                ptr::null(),
                ptr::null(),
            );

            let shaderSource = fragmentSource.data;
            gl::ShaderBinary(
                1,
                &fragmentShader,
                gl::SHADER_BINARY_FORMAT_SPIR_V,
                shaderSource.as_ptr() as _,
                shaderSource.len() as i32,
            );
            let entry = CString::new("main").unwrap();
            gl::SpecializeShader(fragmentShader, entry.as_ptr(), 0, ptr::null(), ptr::null());

            let program = gl::CreateProgram();

            gl::AttachShader(program, vertexShader);
            gl::AttachShader(program, fragmentShader);
            gl::LinkProgram(program);

            program
        }
    }
    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.program);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::resources::Shader;

    #[test]
    pub fn findShader() {
        let data = Shader::findShader("standard");
    }
}
