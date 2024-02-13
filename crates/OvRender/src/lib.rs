#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

pub use gl;
use rust_embed::RustEmbed;

pub mod core;
mod context;
pub mod settings;
pub mod buffers;

pub mod resources;
pub mod geometry;
#[derive(RustEmbed)]
#[folder = "assets"]
struct Asset;


