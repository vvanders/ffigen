// THIS IS AN AUTOGENERATED FILE
// DO NOT MODIFY!

extern crate ffigen;

#[no_mangle]
pub extern fn test_string_marshal(p: *const u8) -> *const u8 {
	let p_shadow = ffigen::marshal::cstr_to_string(p);
	let invoke_result = super::test_string(p_shadow);

	ffigen::marshal::allocate_cstr(&invoke_result)
}
#[no_mangle]
pub extern fn test_string_ref_marshal(p: *const u8) -> *const u8 {
	let p_shadow = ffigen::marshal::cstr_to_string(p);
	let invoke_result = super::test_string_ref(&p_shadow);

	ffigen::marshal::allocate_cstr(&invoke_result)
}
#[no_mangle]
pub extern fn test_str_ref_marshal(p: *const u8) -> *const u8 {
	let p_shadow = ffigen::marshal::cstr_to_string(p);
	let invoke_result = super::test_str_ref(&p_shadow.as_ref());

	ffigen::marshal::allocate_cstr(&invoke_result)
}
#[no_mangle]
pub extern fn test_multi_str_marshal(p1: *const u8, p2: *const u8, p3: *const u8) -> *const u8 {
	let p1_shadow = ffigen::marshal::cstr_to_string(p1);
	let p2_shadow = ffigen::marshal::cstr_to_string(p2);
	let p3_shadow = ffigen::marshal::cstr_to_string(p3);
	let invoke_result = super::two::test_multi_str(p1_shadow, &p2_shadow, &p3_shadow.as_ref());

	ffigen::marshal::allocate_cstr(&invoke_result)
}
