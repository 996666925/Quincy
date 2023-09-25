use nalgebra::Vector2;
use obj::{load_obj, Obj, TexturedVertex};
use std::io::BufReader;
use std::{error::Error, fs::File};
use OvRender::geometry::Vertex;
use OvRender::resources::MeshFile;

fn main() -> Result<(), Box<dyn Error>> {
    let input = BufReader::new(File::open("monkey.obj").expect("找不到文件"));
    let obj: Obj<TexturedVertex, u16> = load_obj(input)?;

    let mut indices: Vec<u16> = obj.indices.clone();
    let mut vertices = obj
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
    let meshFile = MeshFile::new(vertices, indices);

    let mesh = ron::to_string(&meshFile)?;
    std::fs::write("monkey.mesh", mesh)?;
    // print!("{}", mesh);
    Ok(())
}
