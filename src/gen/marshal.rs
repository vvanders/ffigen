use std;
use std::path::Path;

use parser;
use parser::cargo;
use gen::util;

pub fn gen(exports: &Vec<parser::FuncDecl>, package_info: &cargo::Info, dest: &Path) -> std::io::Result<()> {
    let mut content = String::new();

    for func in exports {
        if let Some(func_name) = get_mangled_fn(func) {
            append_func(func, &func_name, &mut content);
        } 
    }

    if content.len() > 0 {
        let output = dest.join("mod.rs");

        util::write_source(&content, &output)
    } else {
        Ok(())
    }
}

pub fn get_mangled_fn(func: &parser::FuncDecl) -> Option<String> {
    let marshaled_name = format!("{}_marshal", func.name);
    let should_marshal = |ty: &parser::Type| {
        match *ty {
            parser::Type::String | parser::Type::StringRef => true,
            parser::Type::Str | parser::Type::StrRef => true,
            _ => false
        }
    };

    let arg_marshal = func.args.iter()
        .any(|ref arg| should_marshal(&arg.ty));

    let ret_marshal = match func.ret {
        parser::ReturnType::Void => false,
        parser::ReturnType::Type(t) => should_marshal(&t)
    };

    if arg_marshal || ret_marshal {
        Some(marshaled_name)
    } else {
        None
    }
}

fn append_func(func: &parser::FuncDecl, func_name: &String, content: &mut String) {
    let mut params = String::new();

    for arg in &func.args {
        if params.len() > 0 {
            params.push_str(", ");
        }

        let param_dec = format!("{}: {}", arg.name, translate_type(arg.ty));

        params.push_str(param_dec.as_ref());
    }

    let func_decl = format!("pub extern fn {}({});\n", func_name, params);

    content.push_str(func_decl.as_ref());
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
		parser::Type::String => "String",
        parser::Type::StringRef => "&String",
        parser::Type::Str => "str",
        parser::Type::StrRef => "&str"
    }
}