use cc::Build;
use std::{env, path::PathBuf};

const HEADER_FILES: &[&str] = &[
    "imath.h",
    "imrat.h",
    "iprime.h",
];

const SRC_FILES: &[&str] = &[
    "imath.c",
    "imrat.c",
    "iprime.c",
];

fn main() {
    let root_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .canonicalize()
        .unwrap();
    let src = root_dir.join("vendor").join("imath");

    let headers: Vec<_> = HEADER_FILES.iter().map(|head| src.join(head)).collect();
    let sources: Vec<_> = SRC_FILES.iter().map(|head| src.join(head)).collect();

    let mut source_builder = Build::new();
    
    for path in &sources {
        source_builder.file(path);
    }

    source_builder.include(&src)
        .compile("imath");


    // Tell cargo to invalidate the built crate whenever the headers change
    for path in &headers {
        println!("cargo:rerun-if-changed={}", path.display());
    }

    let mut bindings_builder = bindgen::builder();

    // Add all headers
    for path in &headers {
        bindings_builder = bindings_builder.header(format!("{}", path.display()));
    }

    // Parse and generate source
    let bindings = bindings_builder.generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}