


fn main() {
    println!(include_str!(concat!(env!("OUT_DIR"), "/bindings.h")));
}