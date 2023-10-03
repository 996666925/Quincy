use std::io::Cursor;

use rust_embed::EmbeddedFile;
use serde::{Deserialize, Serialize};

use super::Resource;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Texture {
    name: String,
    id: u32,
    height: u32,
    width: u32,
}

impl Texture {
    pub fn new(res: Resource) -> Self {
        let Resource { file, name } = res;
        unsafe {
            let mut texture = 0;
            gl::CreateTextures(gl::TEXTURE_2D, 1, &mut texture);
            let dds = ddsfile::Dds::read(&mut Cursor::new(&file.data)).expect("无法加载材质");
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::CompressedTexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::COMPRESSED_RGBA_BPTC_UNORM,
                dds.get_width() as i32,
                dds.get_height() as i32,
                0,
                dds.get_main_texture_size().unwrap() as i32,
                dds.data.as_ptr() as _,
            );

            gl::GenerateMipmap(gl::TEXTURE_2D);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as _);

            gl::BindTexture(gl::TEXTURE_2D, 0);
            return Self {
                name,
                id: texture,
                height: 0,
                width: 0,
            };
        }
    }
    pub fn bind(&self, index: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + index);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn getName(&self) -> &str {
        &self.name
    }
}
