use std::marker::PhantomData;
use std::mem::size_of;

use serde::{Serialize, Deserialize};

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct IndexBuffer {
    pub buffer: u32,
}

impl IndexBuffer {
    pub fn new<E: bytemuck::Pod>(value: &[E]) -> Self {
        unsafe {
            let mut buffer = 0;
            gl::CreateBuffers(1, &mut buffer);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, buffer);
            let data: &[u8] = bytemuck::cast_slice(value);
            gl::NamedBufferStorage(
                buffer,
                data.len() as isize,
                data.as_ptr() as _,
                gl::DYNAMIC_STORAGE_BIT,
            );
            Self { buffer }
        }
    }
    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.buffer);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }

}

