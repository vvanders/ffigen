﻿use std;
use std::path::Path;

use parser;
use gen::util;

pub fn gen(exports: &Vec<parser::FuncDecl>, dest: &Path) -> std::io::Result<()> {
    println!("Looking for functions that require marshaling");

    let mut content = String::new();

    for func in exports {
        if let Some(func_name) = get_mangled_fn(func) {
            append_func(func, &func_name, &mut content);
        } 
    }

    if content.len() > 0 {
        println!("Generating marshaling functions for rust");

        let output = dest.join("ffigen").join("mod.rs");

        let final_content = append_header(&content);

        util::write_source(&final_content, &output)
    } else {
        Ok(())
    }
}

fn should_marshal(ty: &parser::Type) -> bool {
    match *ty {
        parser::Type::String
        | parser::Type::StringRef
        | parser::Type::StrRef => true,
        _ => false
    }
}

pub fn get_mangled_fn(func: &parser::FuncDecl) -> Option<String> {
    let marshaled_name = format!("{}_marshal", func.name);
    

    let arg_marshal = func.args.iter()
        .any(|ref arg| should_marshal(&arg.ty));

    let ret_marshal = has_marshaled_ret_value(func);

    if arg_marshal || ret_marshal {
        Some(marshaled_name)
    } else {
        None
    }
}

pub fn has_marshaled_ret_value(func: &parser::FuncDecl) -> bool {
    match func.ret {
        parser::ReturnType::Void => false,
        parser::ReturnType::Type(t) => should_marshal(&t)
    }
}

fn get_invoke_args(args: &Vec<parser::Arg>) -> String {
    args.iter()
        .map(|arg| format!("{}: {}", arg.name, translate_type(arg.ty)) )
        .fold(String::new(), |acc, arg| {
            match acc.len() {
                0 => arg,
                _ => format!("{}, {}", acc, arg)
            }
        })
}

fn is_shadowed(ty: parser::Type) -> bool {
    match ty {
        parser::Type::String
        | parser::Type::StringRef
        | parser::Type::StrRef => true,
        _ => false
    }
}

fn get_shadowed_name(arg: &parser::Arg) -> String {
    match is_shadowed(arg.ty) {
        true => format!("{}_shadow", arg.name),
        false => arg.name.clone()
    }
}

fn get_call_arg(arg: &parser::Arg) -> String {
    match arg.ty {
        parser::Type::StringRef => format!("&{}", get_shadowed_name(arg)),
        parser::Type::StrRef => format!("&{}.as_ref()", get_shadowed_name(arg)),
        parser::Type::String | _ => format!("{}", get_shadowed_name(arg))
    }
}

fn get_call_args(args: &Vec<parser::Arg>) -> String {
    args.iter()
        .map(|arg| get_call_arg(arg))
        .fold(String::new(), |acc, arg| {
            match acc.len() {
                0 => arg,
                _ => format!("{}, {}", acc, arg)
            }
        })
}

fn get_shadow_decl(arg: &parser::Arg) -> Option<String> {
    match is_shadowed(arg.ty) {
        true => match arg.ty {
            parser::Type::StringRef
            | parser::Type::String
            | parser::Type::StrRef => Some(format!("let {} = ffigen::marshal::cstr_to_string({});", get_shadowed_name(arg), arg.name)),
            _=> None
        },
        false => None
    }
}

fn get_shadow_statements(args: &Vec<parser::Arg>) -> String {
    args.iter()
        .filter_map(|arg| get_shadow_decl(arg))
        .map(|arg| format!("\t{}\n", arg))
        .fold(String::new(), |acc, decl| format!("{}{}", acc, decl))
}

fn get_func_call(func: &parser::FuncDecl) -> String {
    let module_append = match func.module.len() {
        0 => func.module.clone(),
        _ => format!("{}::", func.module)
    };

    match func.ret {
        parser::ReturnType::Void => format!("\tsuper::{}{}({});\n", module_append, func.name, get_call_args(&func.args)),
        parser::ReturnType::Type(ty) => match ty {
            parser::Type::String => {
                let invoke = format!("let invoke_result = super::{}{}({});", module_append, func.name, get_call_args(&func.args));
                let alloc_result = match ty {
                    parser::Type::String => format!("ffigen::marshal::allocate_cstr(&invoke_result)"),
                    _ => panic!("Not a marshaled type")
                };

                format!("\t{}\n\n\t{}\n", invoke, alloc_result)
            },
            _ => format!("\tsuper::{}{}({});\n", func.module, func.name, get_call_args(&func.args))
        }
    }
}

fn append_func(func: &parser::FuncDecl, func_name: &String, content: &mut String) {
    let params = get_invoke_args(&func.args);
    let mut func_body = get_shadow_statements(&func.args);
    let func_call = get_func_call(func);

    let func_decl = match func.ret {
        parser::ReturnType::Void => format!("#[no_mangle]\npub extern fn {}({}) {{\n", func_name, params),
        parser::ReturnType::Type(t) => format!("#[no_mangle]\npub extern fn {}({}) -> {} {{\n", func_name, params, translate_type(t))
    };

    func_body.push_str(func_call.as_ref());

    content.push_str(func_decl.as_ref());
    content.push_str(func_body.as_ref());
    content.push_str("}\n");
}

fn append_header(content: &String) -> String {
    let mut final_content = r"// THIS IS AN AUTOGENERATED FILE
// DO NOT MODIFY!

extern crate ffigen;

".to_string();

    final_content.push_str(content.as_ref());

    final_content
}

fn translate_type(ty: parser::Type) -> &'static str {
    match ty {
        parser::Type::U32 => "u32",
        parser::Type::U16 => "u16",
        parser::Type::U8 => "u8",
        parser::Type::I32 => "i32",
        parser::Type::I16 => "i16",
        parser::Type::I8 => "i8",
        parser::Type::F32 => "f32",
        parser::Type::F64 => "f64",
        parser::Type::Boolean => "bool",
		parser::Type::String => "*const u8",
        parser::Type::StringRef => "*const u8",
        parser::Type::StrRef => "*const u8"
    }
}

/*****
*   Tests
*****/
#[test]
fn test_mangle() {
    let no_mangle = parser::FuncDecl { 
        name: "Foo".to_string(),
        ret: parser::ReturnType::Void,
        args: vec![parser::Arg { name: "arg1".to_string(), ty: parser::Type::U32 }],
        module: "foo::bar".to_string()
    };

    let mangle_ret = parser::FuncDecl { 
        name: "Foo".to_string(),
        ret: parser::ReturnType::Type(parser::Type::String),
        args: vec![parser::Arg { name: "arg1".to_string(), ty: parser::Type::U32 }],
        module: "foo::bar".to_string()
    };

    let mangle_arg = parser::FuncDecl { 
        name: "Foo".to_string(),
        ret: parser::ReturnType::Void,
        args: vec![parser::Arg { name: "arg1".to_string(), ty: parser::Type::StringRef }],
        module: "foo::bar".to_string()
    };

    if let Some(_) = get_mangled_fn(&no_mangle) {
        panic!("Non-mangled function returned mangled name".to_string());
    }

    if let None = get_mangled_fn(&mangle_ret) {
        panic!("Mangled return type returned non-mangled name".to_string());
    }

    if let None = get_mangled_fn(&mangle_arg) {
        panic!("Mangled arg type returned non-mangled name".to_string());
    }
}

#[test]
fn test_args() {
    let mult_args = vec![
        parser::Arg { name: "foo".to_string(), ty: parser::Type::U32 },
        parser::Arg { name: "bar".to_string(), ty: parser::Type::StringRef }
    ];

    assert_eq!(get_invoke_args(&mult_args), format!("foo: {}, bar: {}", translate_type(mult_args[0].ty), translate_type(mult_args[1].ty)));

    let single_arg = vec![ parser::Arg { name: "foo".to_string(), ty: parser::Type::Boolean }];

    assert_eq!(get_invoke_args(&single_arg), format!("foo: {}", translate_type(single_arg[0].ty)));
}

#[test]
fn test_invoke() {
    let mult_args = vec![
        parser::Arg { name: "foo".to_string(), ty: parser::Type::U32 },
        parser::Arg { name: "bar".to_string(), ty: parser::Type::StringRef }
    ];

    assert_eq!(get_call_args(&mult_args), format!("{}, {}", get_call_arg(&mult_args[0]), get_call_arg(&mult_args[1])));

    let single_arg = vec![ parser::Arg { name: "foo".to_string(), ty: parser::Type::Boolean }];

    assert_eq!(get_call_args(&single_arg), get_call_arg(&single_arg[0]));

    assert_eq!("foo_shadow", get_call_arg(&parser::Arg { name: "foo".to_string(), ty: parser::Type::String }));
    assert_eq!("&foo_shadow", get_call_arg(&parser::Arg { name: "foo".to_string(), ty: parser::Type::StringRef }));
    assert_eq!("&foo_shadow.as_ref()", get_call_arg(&parser::Arg { name: "foo".to_string(), ty: parser::Type::StrRef }));
}

#[test]
fn test_shadow_decl() {
    let mult_args = vec![
        parser::Arg { name: "foo".to_string(), ty: parser::Type::U32 },
        parser::Arg { name: "bar".to_string(), ty: parser::Type::StringRef }
    ];

    assert_eq!(None, get_shadow_decl(&mult_args[0]));
    assert_eq!(Some(get_shadow_decl(&mult_args[1]).unwrap()), get_shadow_decl(&mult_args[1]));

    assert_eq!(Some("let foo_shadow = ffigen::marshal::cstr_to_string(foo);".to_string()), get_shadow_decl(&parser::Arg { name: "foo".to_string(), ty: parser::Type::String }));
    assert_eq!(Some("let foo_shadow = ffigen::marshal::cstr_to_string(foo);".to_string()), get_shadow_decl(&parser::Arg { name: "foo".to_string(), ty: parser::Type::StringRef }));
    assert_eq!(Some("let foo_shadow = ffigen::marshal::cstr_to_string(foo);".to_string()), get_shadow_decl(&parser::Arg { name: "foo".to_string(), ty: parser::Type::StrRef }));
}