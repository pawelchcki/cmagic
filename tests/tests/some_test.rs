use std::process::Command;

fn run_test_file(bin: &str) {
    let status = Command::new(bin)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();

    if !status.success(){
        panic!("c test: {} - FAILED", bin);
    } 
}

#[test]
fn test_a() {
    run_test_file(concat!(env!("OUT_DIR"), "/test.bin"));
}
