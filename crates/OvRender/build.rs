use std::fmt::format;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::process::Command;

fn main() {
    let path = env!("CARGO_MANIFEST_DIR");
    let path = format!("{}/assets/shader/src", path);
    println!("path:{}", path);
    let dir = fs::read_dir(path).unwrap();
    // glslangValidator.exe -G -o ./public/shader/shader.vert.spv ./public/shader/shader.vert --target-env opengl

    for item in dir {
        let path = item.unwrap().path().display().to_string();
        println!("shader:{path}");
        let output = path.replace("src", "output");
        Command::new("glslangValidator.exe")
            .args(&["-G", "-o", &format!("{}.spv", output), &path, "--target-env", "opengl"]).output().unwrap();
    }
    println!("cargo:rerun-if-changed=assets/shader/src")
}