use std::marker::PhantomData;
use std::mem::size_of;
use std::ptr;


pub struct UniformBuffer<T> {
    buffer: u32,
    index: u32,
    size: usize,
    maker: PhantomData<T>,
}

impl<T: bytemuck::Pod> UniformBuffer<T> {
    pub fn new(index: u32) -> Self {
        unsafe {
            let mut buffer = 0;
            gl::CreateBuffers(1, &mut buffer);
            gl::BindBuffer(gl::UNIFORM_BUFFER, buffer);
            gl::BindBufferBase(gl::UNIFORM_BUFFER, index, buffer);
            let size = size_of::<T>();
            gl::NamedBufferStorage(buffer, size as _, ptr::null() as _, gl::DYNAMIC_STORAGE_BIT);
            gl::BindBuffer(gl::UNIFORM_BUFFER, 0);
            Self {
                index,
                buffer,
                size,
                maker: PhantomData,
            }
        }
    }
    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::UNIFORM_BUFFER, self.buffer);
            gl::BindBufferBase(gl::UNIFORM_BUFFER, self.index, self.buffer);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::UNIFORM_BUFFER, 0);
        }
    }

    pub fn buffer(&self) -> u32 {
        self.buffer
    }

    pub fn setSubData<E>(&self, offset: usize, data: &[E])
    where
        E: bytemuck::Pod,
    {
        self.bind();

        let data: &[u8] = bytemuck::cast_slice(data);
        unsafe {
            gl::NamedBufferSubData(
                self.buffer,
                offset as _,
                data.len() as _,
                data.as_ptr() as _,
            );
        }
        self.unbind();
    }

    pub fn getData(&self) -> T {
        unsafe {
            self.bind();
            let mut ptr: *mut T = gl::MapNamedBuffer(self.buffer, gl::DYNAMIC_STORAGE_BIT) as _;
            let value = ptr.read().clone();
            gl::UnmapNamedBuffer(self.buffer);
            self.unbind();
            return value;
        }
    }
}

