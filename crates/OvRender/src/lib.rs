#![allow(non_snake_case)]

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