pub extern fn foo(p: u32) -> u32 {
    p
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