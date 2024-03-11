use std::io::Cursor;

use rust_embed::EmbeddedFile;
use serde::{Deserialize, Serialize};

use crate::Asset;

use super::Resource;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum TextureKind {
    Line { length: u32 },

    Rectangle { width: u32, height: u32 },

    Cube { width: u32, height: u32 },

    Volume { width: u32, height: u32, depth: u32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Texture {
    name: String,
    id: u32,
    kind: TextureKind,
}

impl Default for Texture {
    fn default() -> Self {
        Self {
            name: String::new(),
            id: 0,
            kind: TextureKind::Rectangle {
                width: 0,
                height: 0,
            },
        }
    }
}

impl Texture {
    pub fn skybox() -> Self {
        let textures = vec![
            Asset::get("skybox/back.dds").unwrap(),
            Asset::get("skybox/front.dds").unwrap(),
            Asset::get("skybox/top.dds").unwrap(),
            Asset::get("skybox/bottom.dds").unwrap(),
            Asset::get("skybox/left.dds").unwrap(),
            Asset::get("skybox/right.dds").unwrap(),
        ];

        let textures = textures
            .iter()
            .map(|file| file.data.as_ref())
            .collect::<Vec<&[u8]>>();

        Self::from_bytes(
            textures,
            TextureKind::Cube {
                width: 0,
                height: 0,
            },
        )
    }

    pub fn new(res: Resource) -> Self {
        let Resource { file, name } = res;
        unsafe {
            let mut texture = 0;
            gl::CreateTextures(gl::TEXTURE_2D, 1, &mut texture);
            let dds = ddsfile::Dds::read(&mut Cursor::new(&file.data)).expect("无法加载材质");

            gl::TextureStorage2D(
                texture,
                1,
                gl::COMPRESSED_RGBA_BPTC_UNORM,
                dds.get_width() as i32,
                dds.get_height() as i32,
            );
            gl::CompressedTextureSubImage2D(
                texture,
                0,
                0,
                0,
                dds.get_width() as i32,
                dds.get_height() as i32,
                gl::COMPRESSED_RGBA_BPTC_UNORM,
                dds.get_main_texture_size().unwrap() as i32,
                dds.data.as_ptr() as _,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as _);

            return Self {
                name,
                id: texture,
                kind: TextureKind::Rectangle {
                    width: dds.get_width(),
                    height: dds.get_height(),
                },
            };
        }
    }

    pub fn from_bytes(bytes: Vec<&[u8]>, kind: TextureKind) -> Self {
        unsafe {
            let mut texture = 0;

            match kind {
                TextureKind::Rectangle { width, height } => {
                    gl::CreateTextures(gl::TEXTURE_2D, 1, &mut texture);
                    gl::BindTexture(gl::TEXTURE_2D, texture);

                    let dds = ddsfile::Dds::read(&mut Cursor::new(bytes[0])).expect("无法加载材质");

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
                }
                TextureKind::Cube {
                    mut width,
                    mut height,
                } => {
                    gl::CreateTextures(gl::TEXTURE_CUBE_MAP, 1, &mut texture);
                    gl::BindTexture(gl::TEXTURE_CUBE_MAP, texture);
                    let mut dds =
                        ddsfile::Dds::read(&mut Cursor::new(bytes[0])).expect("无法加载材质");

                    gl::TextureStorage2D(
                        texture,
                        1,
                        gl::COMPRESSED_RGBA_BPTC_UNORM,
                        dds.get_width() as i32,
                        dds.get_height() as i32,
                    );

                    let mut index = 0;
                    for data in &bytes {
                        dds = ddsfile::Dds::read(&mut Cursor::new(data)).expect("无法加载材质");

                        gl::CompressedTextureSubImage3D(
                            texture,
                            0,
                            0,
                            0,
                            index as _,
                            dds.get_width() as i32,
                            dds.get_height() as i32,
                            1,
                            gl::COMPRESSED_RGBA_BPTC_UNORM,
                            dds.get_main_texture_size().unwrap() as i32,
                            dds.data.as_ptr() as _,
                        );

                        index += 1;
                        // width = dds.get_width();
                        // height = dds.get_height();
                    }

                    gl::TexParameteri(
                        gl::TEXTURE_CUBE_MAP,
                        gl::TEXTURE_MIN_FILTER,
                        gl::LINEAR as _,
                    );
                    gl::TexParameteri(
                        gl::TEXTURE_CUBE_MAP,
                        gl::TEXTURE_MAG_FILTER,
                        gl::LINEAR as _,
                    );
                    gl::TexParameteri(
                        gl::TEXTURE_CUBE_MAP,
                        gl::TEXTURE_WRAP_S,
                        gl::CLAMP_TO_EDGE as _,
                    );
                    gl::TexParameteri(
                        gl::TEXTURE_CUBE_MAP,
                        gl::TEXTURE_WRAP_T,
                        gl::CLAMP_TO_EDGE as _,
                    );
                    gl::TexParameteri(
                        gl::TEXTURE_CUBE_MAP,
                        gl::TEXTURE_WRAP_R,
                        gl::CLAMP_TO_EDGE as _,
                    );
                    gl::GenerateMipmap(gl::TEXTURE_CUBE_MAP);
                    // gl::BindTexture(gl::TEXTURE_CUBE_MAP, 0);
                }
                _ => {
                    todo!("目前只支持2D和Cube")
                }
            }

            Self {
                name: "SkyBox".to_string(),
                id: texture,
                kind,
            }
        }
    }

    pub fn bind(&self, index: u32) {
        unsafe {
            gl::BindTextureUnit(index, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
            gl::BindTexture(gl::TEXTURE_CUBE_MAP, 0);
        }
    }

    pub fn getName(&self) -> &str {
        &self.name
    }

    pub fn get_width(&self) -> u32 {
        match self.kind {
            TextureKind::Line { length } => 0,
            TextureKind::Rectangle { width, height } => width,
            TextureKind::Cube { width, height } => width,
            TextureKind::Volume {
                width,
                height,
                depth,
            } => width,
        }
    }

    pub fn get_height(&self) -> u32 {
        match self.kind {
            TextureKind::Line { length } => 0,
            TextureKind::Rectangle { width, height } => height,
            TextureKind::Cube { width, height } => height,
            TextureKind::Volume {
                width,
                height,
                depth,
            } => height,
        }
    }
}
