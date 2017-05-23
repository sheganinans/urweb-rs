extern crate cheddar;

fn main() {
    cheddar::Cheddar::new().expect("could not read manifest")
        .insert_code("#include <urweb.h>").run_build("ffi.h"); }
