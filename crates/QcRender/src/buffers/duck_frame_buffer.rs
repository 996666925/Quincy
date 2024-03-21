use std::marker::PhantomData;
use std::mem::size_of;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuckFrameBuffer {}

impl DuckFrameBuffer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn resize(&self, x: f32, y: f32, width: f32, height: f32) {
        unsafe {
            gl::Enable(gl::SCISSOR_TEST);
            let x = x;
            let y = y;
            let width = width;
            let height = height;
            gl::Scissor(x as _, y as _, width as _, height as _);
            gl::Viewport(x as _, y as _, width as _, height as _);
        }
    }

    /// 没什么卵用
    pub fn bind(&self) {}

    /// 没什么卵用
    pub fn unbind(&self) {}
}
