mod frame_buffer;
mod index_buffer;
mod shader_storagr_buffer;
mod uniform_buffer;
mod vertex_array;
mod vertex_buffer;
mod duck_frame_buffer;

pub use frame_buffer::FrameBuffer;
pub use index_buffer::IndexBuffer;
pub use shader_storagr_buffer::ShaderStorageBuffer;
pub use uniform_buffer::UniformBuffer;
pub use vertex_array::*;
pub use vertex_buffer::VertexBuffer;
pub use duck_frame_buffer::DuckFrameBuffer;

pub enum AccessSpecifier {
    DYNAMIC_STORAGE_BIT = 0x0100,
    MAP_READ_BIT = 0x0001,
    MAP_WRITE_BIT = 0x0002,
    MAP_PERSISTENT_BIT = 0x0040,
    MAP_COHERENT_BIT = 0x0080,
    CLIENT_STORAGE_BIT = 0x0200,
}
