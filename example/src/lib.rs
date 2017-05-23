#![crate_type = "dylib"]

extern crate urweb;
extern crate rand;

use std::ffi::{CString};
use urweb::*;

use rand::{thread_rng, Rng};

#[no_mangle]
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub extern fn uw_Ffi_random_secret(ctx : uw_context, u : uw_unit) -> uw_Basis_string {
    let s: String = thread_rng().gen_ascii_chars().take(50).collect();
    CString::new(s).unwrap().into_raw()
}
