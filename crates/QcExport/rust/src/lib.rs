use nalgebra::Vector2;
use obj::{load_obj, Obj, TexturedVertex};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use QcRender::geometry::Vertex;
use QcRender::resources::MeshFile;

#[cxx::bridge(namespace = "QcExport")]
mod ffi {
    extern "Rust" {

        pub fn create_mesh(url: &str);
    }
}

pub fn create_mesh(url: &str) {
    println!("url:{}", url);

    let path = Path::new(url);
    let input = BufReader::new(File::open(path).expect("找不到文件"));
    let obj: Obj<TexturedVertex, u16> = load_obj(input).unwrap();

    let indices: Vec<u16> = obj.indices.clone();
    let vertices = obj
        .vertices
        .iter()
        .map(|v| {
            Vertex::new(
                v.position.into(),
                Vector2::new(v.texture[0], v.texture[1]),
                v.normal.into(),
            )
        })
        .collect::<Vec<Vertex>>();
    let mesh_file = MeshFile::new(vertices, indices);

    let mesh = ron::to_string(&mesh_file).unwrap();

    let mut path = path.to_path_buf();
    path.set_extension("mesh");

    std::fs::write(path, mesh).unwrap();
}
