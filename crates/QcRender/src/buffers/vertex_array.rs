use crate::buffers::VertexBuffer;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VertexArray {
    value: u32,
}

pub enum Type {
    BYTE = 0x1400,
    UNISGNED_BYTE = 0x1401,
    SHORT = 0x1402,
    UNSIGNED_SHORT = 0x1403,
    INT = 0x1404,
    UNSIGNED_INT = 0x1405,
    FLOAT = 0x1406,
    DOUBLE = 0x140A,
}

impl VertexArray {
    pub fn new() -> Self {
        unsafe {
            let mut value = 0;
            gl::CreateVertexArrays(1, &mut value);
            gl::BindVertexArray(value);
            Self { value }
        }
    }
    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.value);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    ///注意此时没有绑定vao、vbo
    ///notice that it doesn't bind the vao and vbo now
    pub fn bindAttribute(&self, index: u32, size: i32, r#type: Type, stride: usize, offset: usize) {
        unsafe {
            gl::VertexAttribPointer(
                index,
                size,
                r#type as _,
                false as u8,
                stride as _,
                offset as _,
            );
            gl::EnableVertexAttribArray(index);
        }
    }
}
