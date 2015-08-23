extern crate syntex_syntax;

use std::ops::Deref;

use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;

use parser;

pub fn gen(exports: &Vec<parser::FuncDecl>, dest: &Path) {
    println!("Creating {:?}", dest);
    if let Err(e) = fs::create_dir(dest) {
        println!("Unable to create dir {:?} {}", dest, e)
    }

    let out_path = dest.join("mod.cs");
    let file = match fs::File::create(&out_path) {
        Ok(f) => f,
        Err(e) => panic!("Unable to open file {:?}", e)
    };

    let mut content = write_header();

    for export in exports {
        write_export(&mut content, export);
    }

    write_footer(&mut content);

    let mut writer = io::BufWriter::new(&file);
    let bytes = content.into_bytes();

    if let Err(e) = writer.write_all(bytes.into_boxed_slice().deref()) {
        panic!("Unable to write file {:?} {}", out_path, e);
    }
}

fn write_header() -> String {
    "namespace rust {\n".to_string()
}

fn write_export(content: &mut String, export: &parser::FuncDecl) {
    content.push_str("\t[DllImport(\"ffi_sample.dll\")]\n");
    
    let func_name = &export.name;
    let mut params = "".to_string();

    for param in &export.args {
        if params.len() > 0 {
            params.push_str(", ");
        }

        let param_dec = format!("{} {}", translate_type(param.ty), param.name);

        params.push_str(param_dec.as_ref());
    }

    let func_decl = format!("\t{} {}({});\n", translate_ret_type(export.ret), func_name, params);

    content.push_str(func_decl.as_ref());
}

fn translate_ret_type(ty: parser::ReturnType) -> &'static str {
    match ty {
        parser::ReturnType::Void => "void",
        parser::ReturnType::Type(t) => translate_type(t)
    }
}

fn translate_type(ty: parser::Type) -> &'static str {
    match ty {
        parser::Type::U32 => "uint",
        parser::Type::U16 => "ushort",
        parser::Type::U8 => "ubyte",
        parser::Type::I32 => "int",
        parser::Type::I16 => "short",
        parser::Type::I8 => "byte",
        parser::Type::F32 => "float",
        parser::Type::F64 => "double",
        parser::Type::Boolean => "bool"
    }
}

fn write_footer(content: &mut String) {
    content.push_str("}");
}