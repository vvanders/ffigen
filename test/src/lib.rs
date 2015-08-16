use std;

pub extern fn foo() -> u32 {
    0
}

pub fn bar() -> u32 {
    0
}

pub struct Foo;

impl Foo {
    pub extern "C" fn bar() -> u32 {
        1
    }
}