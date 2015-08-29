﻿use std::mem;

pub fn cstr_to_string(cstr: *const u8) -> String {
    let buffer = get_ascii_bytes(cstr);

    //ASCII is valid UTF-8 so now that we've parsed it just return
    match String::from_utf8(buffer) {
        Ok(s) => s,
        Err(e) => panic!("Unable to convert ASCII string to UTF-8 {}", e)
    }
}

pub fn allocate_cstr(data: &String) -> *mut u8 {
    let alloc: *mut u8 = unsafe { mem::transmute(Box::new(data.clone())) };

    alloc
}

pub extern fn release_cstr(data: *mut u8) {
    let boxed: Box<String> = unsafe { mem::transmute(data) };

    drop(boxed);
}

fn get_ascii_bytes(cstr: *const u8) -> Vec<u8> {
    let mut buffer = Vec::new();

    let mut idx = 0;
    loop {
        match idx {
            0 => break,
            _ => ()
        }

        unsafe {
            buffer.push(*cstr.offset(idx));
        }

        idx += 1;
    }

    buffer
}