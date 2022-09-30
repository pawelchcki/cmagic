use std::{os::raw::c_char};

const EXAMPLE_FN: &str = "example_fn\0";

#[no_mangle]
pub extern "C" fn example_fn() ->  *const c_char {
    EXAMPLE_FN.as_ptr() as *const c_char
}

