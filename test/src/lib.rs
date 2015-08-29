pub mod one;
pub mod two;

mod ffigen;

#[no_mangle]
pub extern fn foo(p: u32) -> u32 {
    p
}

#[no_mangle]
pub extern fn foostr(p: &String, p2: u32) {
	println!("{}", p);
}

#[no_mangle]
pub extern fn bar() -> u32 {
    0
}