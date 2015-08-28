extern crate syntex_syntax;

use std;
use std::path::Path;

use parser;
use parser::cargo;
use gen::util;

pub fn gen(exports: &Vec<parser::FuncDecl>, package_info: &cargo::Info, dest: &Path) -> std::io::Result<()> {
    if !package_info.is_dynamic {
        panic!("Unable to export {} because library is not dynamic", package_info.name);
    }

    let out_path = dest.join("mod.cs");

    let mut content = write_header(package_info);

    for export in exports {
        write_export(&mut content, export, package_info);
    }

    write_footer(&mut content);

    util::write_source(&content, &out_path)
}

fn write_header(package_info: &cargo::Info) -> String {
    format!(
r"using System.Runtime.InteropServices;

namespace rust {{
    class {} {{
", package_info.name)
}

fn write_export(content: &mut String, export: &parser::FuncDecl, package_info: &cargo::Info) {
    let import_dec = format!("\t\t[DllImport(\"{}.dll\")]\n", package_info.lib_name);
    content.push_str(import_dec.as_ref());
    
    let func_name = &export.name;
    let mut params = "".to_string();

    for param in &export.args {
        if params.len() > 0 {
            params.push_str(", ");
        }

        let param_dec = format!("{} {}", translate_type(param.ty), param.name);

        params.push_str(param_dec.as_ref());
    }

    let func_decl = format!("\t\tpublic static extern {} {}({});\n", translate_ret_type(export.ret), func_name, params);

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
        parser::Type::Boolean => "bool",
		parser::Type::String => "string",
        parser::Type::StringRef => "string",
        parser::Type::Str => "string",
        parser::Type::StrRef => "string"
    }
}

fn write_footer(content: &mut String) {
    content.push_str(
r"    }
}");
}