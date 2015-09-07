use std;

use parser;
use parser::cargo;
use gen::util;
use gen::marshal;

pub fn gen(exports: &Vec<parser::FuncDecl>, package_info: &cargo::Info, opts: &Vec<::Config>) -> std::io::Result<()> {
    if !package_info.is_dynamic {
        panic!("Unable to export {} because library is not dynamic", package_info.name);
    }

    let out_path = util::get_output_dir(opts, &package_info.crate_root).join("Module.cs");

    let mut content = write_header(package_info);

    for export in exports {
        write_export(&mut content, export, package_info);
    }

    write_footer(&mut content);

    util::write_source(&content, &out_path)
}

fn write_header(package_info: &cargo::Info) -> String {
    format!(
r#"using System;
using System.Runtime.InteropServices;

namespace rust {{
    class ffiutils {{
        [DllImport("{}.dll")]
        private static extern void release_cstr(IntPtr strptr);

        //Creates a string from an allocated c string, consuming(releasing) the backing allocation
        public static string consume_cstr(IntPtr strptr) {{
            string str = Marshal.PtrToStringAnsi(strptr);
            release_cstr(strptr);

            return str;
        }}
    }}

    class {} {{
"#, package_info.lib_name, package_info.name)
}

fn get_import_decl(func: &parser::FuncDecl, package_info: &cargo::Info) -> String {
    match marshal::get_mangled_fn(func) {
        Some(mangled) => format!("[DllImport(\"{}.dll\", EntryPoint=\"{}\")]", package_info.lib_name, mangled),
        None => format!("[DllImport(\"{}.dll\")]", package_info.lib_name)
    }
}

fn get_import_params(func: &parser::FuncDecl) -> String {
    func.args.iter()
        .map(|arg| format!("{} {}", translate_type(arg.ty), arg.name))
        .fold(String::new(), |acc, arg| {
            match acc.len() {
                0 => arg,
                _ => format!("{}, {}", acc, arg)
            }
        })
}

fn export_marshaled_return(func: &parser::FuncDecl, package_info: &cargo::Info) -> String {
    let mangled_fn = match marshal::get_mangled_fn(func) {
        Some(v) => v,
        None => panic!("Expected mangled function name")
    };

    let import_decl = format!("[DllImport(\"{}.dll\")]", package_info.lib_name);
    let import_func = format!("private static extern {} {}({});", translate_ret_type(func.ret), mangled_fn, get_import_params(func));

    let invoke_params = func.args.iter()
        .map(|arg| arg.name.clone())
        .fold(String::new(), |acc, arg| {
            match acc.len() {
                0 => arg,
                _ => format!("{}, {}", acc, arg)
            }
        });

    let marshal_func = format!(r"public static string {}({}) {{
            return ffiutils.consume_cstr({}({}));
        }}

", func.name, get_import_params(func), mangled_fn, invoke_params);

    format!("\t\t{}\n\t\t{}\n\t\t{}", import_decl, import_func, marshal_func)
}

fn get_func_signature(func: &parser::FuncDecl) -> String {
    let params = get_import_params(func);

    format!("public static extern {} {}({});", translate_ret_type(func.ret), func.name, params)
}

fn write_export(content: &mut String, func: &parser::FuncDecl, package_info: &cargo::Info) {
    let func_body = match marshal::has_marshaled_ret_value(func) {
        true => export_marshaled_return(func, package_info),
        false => format!("\t\t{}\n\t\t{}\n\n", get_import_decl(func, package_info), get_func_signature(func))
    };

    //Boolean values are 1 byte size, so append this in order to marshal it correctly
    match func.ret {
        parser::ReturnType::Type(parser::Type::Boolean) => content.push_str("\t\t[return: MarshalAs(UnmanagedType.I1)]\n"),
        _=> ()
    }

    content.push_str(func_body.as_ref());
}

fn translate_ret_type(ty: parser::ReturnType) -> &'static str {
    match ty {
        parser::ReturnType::Void => "void",
        parser::ReturnType::Type(t) => match t {
            parser::Type::String => "System.IntPtr",
            parser::Type::StringRef | parser::Type::StrRef => panic!("Unmarshalable type!"),
            _ => translate_type(t)
        }
    }
}

fn translate_type(ty: parser::Type) -> &'static str {
    match ty {
        parser::Type::U32 => "uint",
        parser::Type::U16 => "ushort",
        parser::Type::U8 => "byte",
        parser::Type::I32 => "int",
        parser::Type::I16 => "short",
        parser::Type::I8 => "sbyte",
        parser::Type::F32 => "float",
        parser::Type::F64 => "double",
        parser::Type::Boolean => "bool",
		parser::Type::String => "string",
        parser::Type::StringRef => "string",
        parser::Type::StrRef => "string"
    }
}

fn write_footer(content: &mut String) {
    content.push_str(
r"    }
}");
}