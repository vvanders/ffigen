pub mod one;
pub mod two;

pub mod ffigen;

//Integer marshaling
#[no_mangle]
pub extern fn test_u8(p: u8) -> u8 {
    p
}

#[no_mangle]
pub extern fn test_u16(p: u16) -> u16 {
    p
}

#[no_mangle]
pub extern fn test_u32(p: u32) -> u32 {
    p
}

#[no_mangle]
pub extern fn test_i8(p: i8) -> i8 {
    p
}

#[no_mangle]
pub extern fn test_i16(p: i16) -> i16 {
    p
}

#[no_mangle]
pub extern fn test_i32(p: i32) -> i32 {
    p
}

//Float marshaling
#[no_mangle]
pub extern fn test_f32(p: f32) -> f32 {
    p
}

#[no_mangle]
pub extern fn test_f64(p: f64) -> f64 {
    p
}

//Boolean marshaling
#[no_mangle]
pub extern fn test_bool(p: bool) -> bool {
    p == true
}

//String marshaling
#[no_mangle]
pub extern fn test_string(p: String) -> String {
    p.clone()
}

#[no_mangle]
pub extern fn test_string_ref(p: &String) -> String {
    p.clone()
}

#[no_mangle]
pub extern fn test_str_ref(p: &str) -> String {
    p.to_string()
}