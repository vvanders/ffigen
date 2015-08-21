extern crate syntex_syntax;

use std::fs;
use std::io;
use std::io::Write;
use std::fmt;
use std::path::Path;
use std::ops::Deref;

use syntex_syntax::ast;
use syntex_syntax::ptr::P;
use syntex_syntax::print;

use parser;

pub fn gen(exports: &Vec<parser::FuncDecl>, dest: &Path) {
    println!("Creating {:?}", dest);
    fs::create_dir(dest);

    let file = match fs::File::create(dest.join("mod.cs")) {
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
    writer.write_all(bytes.into_boxed_slice().deref());
}

fn write_header() -> String {
    "namespace rust {\n".to_string()
}

fn write_export(content: &mut String, export: &parser::FuncDecl) {
    content.push_str("\t[DllImport(\"ffi_sample.dll\")]\n");
    
    let func_name = export.name.as_str();
    let mut params = "".to_string();

    for param in &export.args {
        if params.len() > 0 {
            params.push_str(", ");
        }

        let param_type = translate_type(&param.ty);
        let param_name = print::pprust::pat_to_string(param.pat.deref());
        let param_dec = format!("{} {}", param_type, param_name);

        params.push_str(param_dec.as_ref());
    }

    let ret_type = match export.ret {
        ast::FunctionRetTy::NoReturn(_) => "void".to_string(),
        ast::FunctionRetTy::DefaultReturn(_) => "void".to_string(),
        ast::FunctionRetTy::Return(ref r) => translate_type(r)
    };

    let func_decl = format!("\t{} {}({});\n", ret_type, func_name, params);

    content.push_str(func_decl.as_ref());
}

fn write_footer(content: &mut String) {
    content.push_str("}");
}

fn translate_type(ty: &P<ast::Ty>) -> String {
    let rust_type = print::pprust::to_string(|s| s.print_type(ty));

    match rust_type.as_ref() {
        "u32" => "uint".to_string(),
        _ => panic!("Unknown type {}", rust_type)
    }
}