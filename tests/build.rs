use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

fn exec_and_output_to(bin: &str, output_file: &str) {
    let output = Command::new(bin)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
        .wait_with_output()
        .unwrap();

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);

    let mut f = File::create(out_dir.join(output_file)).unwrap();
    f.write_all(&output.stdout).unwrap();
}

fn compile_test_file(src: &str, bin_name: &str) {
    println!("cargo:rerun-if-changed={}", src);

    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);

    let mut command = cc::Build::new()
        .warnings(true)
        .warnings_into_errors(true)
        .include(out_dir.to_str().unwrap())
        .get_compiler()
        .to_command();
    let command = command
        .arg(src)
        .args(["-o", out_dir.join(bin_name).to_str().unwrap()])
        .arg(env::var("CARGO_STATICLIB_FILE_SOMELIB_somelib").unwrap())
        .args(["-lpthread", "-ldl"]);

    println!("compiling: {:?}", command);

    let status = command
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    if !status.success() {
        panic!("compilation failed");
    }

}

fn main() {
    let cbindgen_render_path = env::var("CARGO_BIN_FILE_SOMELIB_cbindgen").unwrap();
    exec_and_output_to(cbindgen_render_path.as_str(), "somelib.h");

    for (k, v) in env::vars() {
        println!("{}: {}", k, v);
    }

    compile_test_file("tests/test.c", "test.bin");
}
