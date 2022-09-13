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

    let bindings = builder.generate().unwrap();
    bindings.write_to_file("src/sys.rs").unwrap();
}
