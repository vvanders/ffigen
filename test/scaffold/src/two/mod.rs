#[no_mangle]
pub extern fn sub_mod_value(p: u32) -> u32 {
    p
}

#[no_mangle]
pub extern fn test_multi_param_unsigned(p1: u8, p2: u16, p3: u32) -> u32 {
    p1 as u32 + p2 as u32 + p3
}

#[no_mangle]
pub extern fn test_multi_param_signed(p1: i8, p2: i16, p3: i32) -> i32 {
    p1 as i32 + p2 as i32 + p3
}

#[no_mangle]
pub extern fn test_multi_str(p1: String, p2: &String, p3: &str) -> String {
    format!("{}{}{}", p1, p2, p3)
}