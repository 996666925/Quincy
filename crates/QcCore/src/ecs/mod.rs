use QcRender::buffers::UniformBuffer;
use nalgebra::{Matrix4, Vector3};

pub mod component;
pub mod components;
pub mod game_object;
pub mod graph;
pub mod renderer;
pub mod drawable;

pub type MvpUbo = UniformBuffer<([Matrix4<f32>; 3], Vector3<f32>)>;
