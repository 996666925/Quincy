use nalgebra::{Point3, Vector2, Vector3};
use obj::{load_obj, Obj, TexturedVertex};
use std::io::BufReader;
use std::vec;
use std::{error::Error, fs::File};
use OvRender::geometry::Vertex;
use OvRender::resources::MeshFile;

fn main() -> Result<(), Box<dyn Error>> {
    let input = BufReader::new(File::open("qiu.obj").expect("找不到文件"));
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
    std::fs::write("qiu.mesh", mesh)?;
    // print!("{}", mesh);
    Ok(())
}

fn main2() -> Result<(), Box<dyn Error>> {
    let obj_file = std::env::args()
        .skip(1)
        .next()
        .expect("A .obj file to print is required");

    let (models, materials) =
        tobj::load_obj(&obj_file, &tobj::LoadOptions::default()).expect("Failed to OBJ load file");

    let mut indices: Vec<u16> = vec![];
    let mut vertices: Vec<Vertex> = vec![];

    println!("Number of models          = {}", models.len());

    for (i, m) in models.iter().enumerate() {
        let mesh = &m.mesh;
        println!("Number of texcoords          = {}", mesh.texcoords.len());
        println!("Number of indices          = {}", mesh.indices.len());
        println!("Number of positions          = {}", mesh.positions.len());

        indices = mesh.indices.iter().map(|f| *f as u16).collect();

        for vtx in 0..mesh.positions.len() / 3 {
            let position = Point3::new(
                mesh.positions[3 * vtx],
                mesh.positions[3 * vtx + 1],
                mesh.positions[3 * vtx + 2],
            );

            let uv = Vector2::new(mesh.texcoords[2 * vtx], mesh.texcoords[2 * vtx + 1]);
            vertices.push(Vertex::new(position, uv, Vector3::new(0., 0., 0.)));
        }
        let meshFile = MeshFile::new(vertices.clone(), indices);

        let mesh = ron::to_string(&meshFile)?;
        std::fs::write("fangzi.mesh", mesh)?;
    }

    Ok(())
}
