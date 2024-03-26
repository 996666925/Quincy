use crate::Asset;

use crate::resources::uniform_type::UniformType;
use nalgebra::Vector4;
use rust_embed::EmbeddedFile;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ffi::CString;
use std::fmt::format;
use std::ptr;
use std::sync::Mutex;
use QcTools::sync::Lazy;

use super::UniformInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shader {
    name: String,
    program: u32,
    uniforms: HashMap<String, Option<UniformInfo>>,
}

impl Default for Shader {
    fn default() -> Self {
        Self::standard()
    }
}

impl Shader {
    pub fn standard() -> Self {
        Self::new("standard")
    }

    pub fn skybox() -> Self {
        Self::new("skybox")
    }

    pub fn new(name: &str) -> Self {
        //抛弃了ShaderMap
        let (vertex, fragment) = Shader::findShader(name);
        let program = Shader::createProgram(vertex, fragment);
        let mut shader = Self {
            name: name.to_string(),
            program,
            uniforms: HashMap::new(),
        };
        shader.query_uniforms();

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

    pub fn query_uniforms(&mut self) {
        unsafe {
            let mut num = 0;
            let mut uniform_max_size = 0;
            gl::GetProgramiv(
                self.program,
                gl::ACTIVE_UNIFORM_MAX_LENGTH,
                &mut uniform_max_size,
            );
            gl::GetProgramiv(self.program, gl::ACTIVE_UNIFORMS, &mut num);

            for i in 0..num {
                let mut type_ = 0;
                let mut array_size = 0;
                let mut length = 0;

                let mut name = String::with_capacity(uniform_max_size as usize);
                name.extend(std::iter::repeat('\0').take(uniform_max_size as usize));
                gl::GetActiveUniform(
                    self.program,
                    i as _,
                    uniform_max_size,
                    &mut length,
                    &mut array_size,
                    &mut type_,
                    name.as_ptr() as _,
                );
                name.truncate(length as usize);

                if !Self::is_engine_ubo_menber(&name) {
                    match type_.into() {
                        // UniformType::UNIFORM_BOOL => todo!(),
                        // UniformType::UNIFORM_INT => todo!(),
                        // UniformType::UNIFORM_FLOAT => todo!(),
                        // UniformType::UNIFORM_FLOAT_VEC2 => todo!(),
                        // UniformType::UNIFORM_FLOAT_VEC3 => todo!(),
                        UniformType::UNIFORM_FLOAT_VEC4 => {
                            let mut vec = Vector4::identity();
                            self.get_uniform_vec4(&name, &mut vec);
                            self.uniforms.insert(name, Some(UniformInfo::Vec4(vec)));
                        }
                        // UniformType::UNIFORM_FLOAT_MAT4 => todo!(),
                        // UniformType::UNIFORM_DOUBLE_MAT4 => todo!(),
                        // UniformType::UNIFORM_SAMPLER_2D => todo!(),
                        // UniformType::UNIFORM_SAMPLER_CUBE => todo!(),
                        _ => {
                            self.uniforms.insert(name, None);
                        }
                    }
                }
            }
        }
    }

    pub fn is_engine_ubo_menber(name: &str) -> bool {
        name.find("MVP").is_some()
    }

    pub fn set_uniform_vec4(&self, name: &str, vec: &Vector4<f32>) {
        unsafe {
            let name = CString::new(name).unwrap();
            gl::Uniform4f(
                gl::GetUniformLocation(self.program, name.as_ptr() as _),
                vec.x,
                vec.y,
                vec.z,
                vec.w,
            )
        }
    }

    pub fn set_uniform_i32(&self, name: &str, i32: i32) {
        unsafe {
            let name = CString::new(name).unwrap();
            gl::Uniform1i(
                gl::GetUniformLocation(self.program, name.as_ptr() as _),
                i32,
            )
        }
    }

    //     OvMaths::FVector4 OvRendering::Resources::Shader::GetUniformVec4(const std::string& p_name)
    // {
    // 	GLfloat values[4];
    // 	glGetUniformfv(id, GetUniformLocation(p_name), values);
    // 	return reinterpret_cast<OvMaths::FVector4&>(values);
    // }

    pub fn get_uniform_vec4(&self, name: &str, vec: &mut Vector4<f32>) {
        unsafe {
            let name = CString::new(name).unwrap();

            let mut arr = [0f32; 4];
            gl::GetUniformfv(
                self.program,
                gl::GetUniformLocation(self.program, name.as_ptr() as _),
                arr.as_mut_ptr(),
            );
            vec.x = arr[0];
            vec.y = arr[1];
            vec.z = arr[2];
            vec.w = arr[3];
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
