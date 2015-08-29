use std::mem;

pub fn cstr_to_string(cstr: *const u8) -> String {
    let buffer = get_ascii_bytes(cstr);

    //ASCII is valid UTF-8 so now that we've parsed it just return
    match String::from_utf8(buffer) {
        Ok(s) => s,
        Err(e) => panic!("Unable to convert ASCII string to UTF-8 {}", e)
    }
}

pub fn allocate_cstr(data: &String) -> *mut u8 {
    let translated = truncate_utf8_to_ascii(data.as_ref());
    let alloc: *mut u8 = unsafe { mem::transmute(Box::new(translated.clone())) };

    alloc
}

pub extern fn release_cstr(data: *mut u8) {
    let boxed: Box<String> = unsafe { mem::transmute(data) };

    drop(boxed);
}

fn truncate_utf8_to_ascii(data: &str) -> Vec<u8> {
    let mut truncated = Vec::new();

    for utf8 in data.chars() {
        truncated.push(utf8 as u8);
    }

    truncated
}

fn get_ascii_bytes(cstr: *const u8) -> Vec<u8> {
    let mut buffer = Vec::new();

    let mut idx = 0;
    loop {
        let ascii = unsafe { *cstr.offset(idx) };

        match ascii {
            0 => break,
            _ => ()
        }
        
        buffer.push(ascii);

        idx += 1;
    }

    buffer
}