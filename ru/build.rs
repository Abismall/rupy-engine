use std::env;
use std::path::PathBuf;

fn main() {
    let project_root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    let target_dir = match env::var("PROFILE").as_deref() {
        Ok("release") => project_root.join("target").join("release"),
        _ => project_root.join("target").join("debug"),
    };

    let target_static_dir = target_dir.join("static");

    println!(
        "cargo:rustc-env=RUPY_ENGINE_STATIC_DIR={}",
        target_static_dir.display()
    );
    println!(
        "cargo:rustc-env=RUPY_ENGINE_TEXTURES_DIR={}",
        target_static_dir.join("textures").display()
    );
    println!(
        "cargo:rustc-env=RUPY_ENGINE_SHADERS_DIR={}",
        target_static_dir.join("shaders").display()
    );
}
