use std::{env, path::Path};

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(cbindgen::Language::C)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_dir.join("bindings.h"));
}
