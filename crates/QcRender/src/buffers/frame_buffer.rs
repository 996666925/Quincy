use std::marker::PhantomData;
use std::mem::size_of;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameBuffer {
    pub buffer: u32,
    pub render_texture: u32,
    pub depth_stencil_buffer: u32,
}

impl FrameBuffer {
    pub fn new(width: u32, height: u32) -> Self {
        unsafe {
            let mut buffer = 0;
            gl::CreateFramebuffers(1, &mut buffer);
            let mut render_texture = 0;
            gl::CreateTextures(gl::TEXTURE_2D, 1, &mut render_texture);
            let mut depth_stencil_buffer = 0;
            gl::CreateRenderbuffers(1, &mut depth_stencil_buffer);

            gl::BindTexture(gl::TEXTURE_2D, render_texture);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as _);
            gl::BindTexture(gl::TEXTURE_2D, 0);

            let this = Self {
                buffer,
                render_texture,
                depth_stencil_buffer,
            };

            this.bind();
            gl::FramebufferTexture(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                this.render_texture,
                0,
            );

            this.unbind();

            this.resize(width, height);

            this
        }
    }

    pub fn resize(&self, width: u32, height: u32) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.render_texture);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as _,
                width as _,
                height as _,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                std::ptr::null(),
            );

            gl::BindTexture(gl::TEXTURE_2D, 0);


            gl::BindRenderbuffer(gl::RENDERBUFFER, self.depth_stencil_buffer);
            gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH_STENCIL, width as _, height as _);

            gl::BindRenderbuffer(gl::RENDERBUFFER, 0);

            self.bind();
            gl::FramebufferRenderbuffer(
                gl::FRAMEBUFFER,
                gl::DEPTH_ATTACHMENT,
                gl::RENDERBUFFER,
                self.depth_stencil_buffer,
            );
            gl::FramebufferRenderbuffer(
                gl::FRAMEBUFFER,
                gl::STENCIL_ATTACHMENT,
                gl::RENDERBUFFER,
                self.depth_stencil_buffer,
            );

            self.unbind();
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.buffer);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }
}
