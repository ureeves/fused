use std::path::Path;
use std::{env, fs};

use fused_src::Build;

fn main() {
    println!("cargo:rerun-if-changed=Cargo.toml");

    let mut builder = Build::new();
    let artifacts = builder.build();

    let mut builder = bindgen::builder()
        .clang_arg("-DFUSE_USE_VERSION=312")
        .ctypes_prefix("::libc");
    for include in artifacts.includes {
        builder = builder.header(include.to_str().unwrap());
    }

    println!(
        "cargo:rustc-link-search={}",
        artifacts.lib_dir.to_str().unwrap()
    );
    println!("cargo:rustc-link-lib=fuse3");

    let generated_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("generated");

    if !generated_dir.exists() {
        fs::create_dir_all(&generated_dir).unwrap();
    }

    let bindings = builder.generate().unwrap();
    bindings
        .write_to_file(generated_dir.join("sys.rs"))
        .unwrap();
}
