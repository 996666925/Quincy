use std::ffi::c_void;

use log::info;
use QcTools::utils::r#ref::Ref;

use crate::{
    context::driver::Driver,
    resources::Mesh,
    settings::{
        driver_settings::DriverSettings,
        pixel_data::{PixelDataFormat, PixelDataType},
    },
};

use super::DrawParameters;

pub enum PrimitiveMode {
    POINTS = 0x0000,
    LINES = 0x0001,
    LINE_LOOP = 0x0002,
    LINE_STRIP = 0x0003,
    TRIANGLES = 0x0004,
    TRIANGLE_STRIP = 0x0005,
    TRIANGLE_FAN = 0x0006,
    LINES_ADJACENCY = 0x000A,
    LINE_STRIP_ADJACENCY = 0x000B,
    TRIANGLES_ADJACENCY = 0x000C,
    TRIANGLE_STRIP_ADJACENCY = 0x000D,
    PATCHES = 0xE,
}

#[derive(Debug)]
pub struct Renderer {}

impl Renderer {
    pub fn new(settings: DriverSettings) -> Self {
        let mut len = 0;
        unsafe { gl::GetIntegerv(gl::MAX_COLOR_ATTACHMENTS, &mut len) }
        Driver::loadSettings(settings);
        Self {}
    }

    pub fn setClearColor(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            gl::ClearColor(r, g, b, a);
        }
    }

    pub fn clear(&self, colorBuffer: bool, depthBuffer: bool, stencilBuffer: bool) {
        unsafe {
            let mut flags = 0;
            if colorBuffer {
                flags |= gl::COLOR_BUFFER_BIT;
            }
            if depthBuffer {
                flags |= gl::DEPTH_BUFFER_BIT;
            }
            if stencilBuffer {
                flags |= gl::STENCIL_BUFFER_BIT;
            }
            gl::Clear(flags);
        }
    }

    pub fn preDraw(&self, params: DrawParameters) {
        unsafe {
            if params.depth_test {
                gl::Enable(gl::DEPTH_TEST);
            } else {
                gl::Disable(gl::DEPTH_TEST);
            }
        }
    }
    pub fn draw(&self, mesh: &Mesh, mode: PrimitiveMode, instance: u32) {
        if instance > 0 {
            mesh.bind();
            unsafe {
                if mesh.getIndexCount() > 0 {
                    if instance == 1 {
                        gl::DrawElements(
                            mode as _,
                            mesh.getIndexCount(),
                            gl::UNSIGNED_SHORT,
                            std::ptr::null(),
                        );
                    }
                }
            }

            mesh.unbind();
        }
    }

    pub fn set_viewport(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe { gl::Viewport(x, y, width, height) }
    }

    pub fn read_pixels(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        format: PixelDataFormat,
        type_: PixelDataType,
        data: *mut c_void,
    ) {
        unsafe {
            gl::ReadPixels(x, y, width, height, format as _, type_ as _, data);
        }
    }
}
