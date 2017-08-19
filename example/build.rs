extern crate cheddar;

fn main() {
    cheddar::Builder::c99().expect("could not read cargo manifest")
        .name("ffi.h")
        .insert_code("#include <urweb.h>")
        .output_directory(".")
        .run_build(); }
