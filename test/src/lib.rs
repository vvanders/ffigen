#![feature(cstr_memory)]

pub mod one;
pub mod two;

use std::ffi::CString;

#[no_mangle]
pub extern fn foo(p: u32) -> u32 {
    p
}

#[no_mangle]
pub extern fn foostr(p: *const i8) {
	unsafe {
		let cstr = CString::from_ptr(p);
		println!("{}", cstr.to_str().unwrap());
	}
}

#[no_mangle]
pub extern fn bar() -> u32 {
    0
}