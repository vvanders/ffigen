pub mod one;
pub mod two;

#[no_mangle]
pub extern fn foo(p: u32) -> u32 {
    p
}

#[no_mangle]
pub extern fn bar() -> u32 {
    0
}