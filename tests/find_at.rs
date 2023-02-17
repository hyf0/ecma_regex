use std::ffi::c_int;

use ecma_regex::Regex;

#[test]
#[should_panic]
fn should_panic_if_start_out_of_text() {
    Regex::new("hello").unwrap().find_at("hello, world", 99);
}
