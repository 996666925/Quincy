fn main() {
    cxx_build::bridge("src/lib.rs")
        .compile("rustlib");

    println!("cargo:rerun-if-changed=src/lib.rs");
}
