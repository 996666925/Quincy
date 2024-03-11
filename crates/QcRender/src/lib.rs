#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

pub use gl;
use rust_embed::RustEmbed;

pub mod buffers;
mod context;
pub mod core;
pub mod geometry;
pub mod resources;
pub mod settings;
#[derive(RustEmbed)]
#[folder = "assets"]
struct Asset;
