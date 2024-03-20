use std::mem::size_of;
use std::{cell::Cell, marker::PhantomData};

use serde::{Deserialize, Serialize};

use super::AccessSpecifier;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShaderStorageBuffer {
    pub buffer: u32,
    pub binding_point: Cell<u32>,
}

impl ShaderStorageBuffer {
    pub fn new(value: AccessSpecifier) -> Self {
        unsafe {
            let mut buffer = 0;
            gl::CreateBuffers(1, &mut buffer);

            gl::NamedBufferStorage(buffer, 0, std::ptr::null() as _, value as _);

            Self {
                buffer,
                binding_point: Cell::new(0),
            }
        }
    }
    pub fn bind(&self, point: u32) {
        unsafe {
            self.binding_point.set(point);
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, point, self.buffer);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            let point = self.binding_point.take();
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, point, 0);
        }
    }

    pub fn buffer(&self) -> u32 {
        self.buffer
    }
}
