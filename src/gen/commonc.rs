use std::iter::Iterator;

use parser;

pub fn get_header_guard_start(package_name: &String) -> String {
    format!("#ifndef _{}_H_\n#define _{}_H_\n\n", package_name, package_name)
}

pub fn get_header_guard_end(package_name: &String) -> String {
    format!("#endif    //_{}_H_", package_name)
}

pub fn get_namespace_indent(namespace: &String) -> usize {
    namespace.split("::").count()
}

pub fn get_namespace_start(namespace: &String) -> String {
    let split_ns = namespace.split("::");

    let mut content = String::new();

    let mut indent = 0;
    for ns in split_ns {
        for _ in 0..indent {
            content.push_str("\t");
        }

        content.push_str(format!("namespace {} {{\n", ns).as_ref());

        indent += 1;
    }

    content
}

pub fn get_namespace_end(namespace: &String) -> String {
    let mut content = String::new();

    for indent in 0..get_namespace_indent(namespace) {
        for _ in 0..indent {
            content.push_str("\t");
        }

        content.push_str("}\n\n");
    }

    content
}

pub fn get_string_decl() -> String {
    "typedef char* RustString;".to_string()
}

pub fn get_string_funcs() -> String {
    "void release_rust_string(RustString str);".to_string()
}

pub fn indent(content: String) -> String {
    content.split("\n")
        .scan(0, |indent, line| {   //Count our indents and associate them with our lines
            if line.ends_with("}") {
                *indent -= 1;
            }

            let result = (*indent, line);

            if line.ends_with("{") {
                *indent += 1;
            }

            Some(result)
        })
        .map(|(indent, line)| { //Prepend tab based on how much we're indented
            ["\t"].iter().cycle()
                .take(indent)
                .fold(String::new(), |acc, c| acc + c) + line.as_ref()
        })
        //Rejoint the lines
        .fold(String::new(), |acc, line| match acc.len() {
            0 => line,
            _ => acc + "\n" + line.as_ref()
        })
}

pub fn translate_return_type(ret: parser::ReturnType) -> &'static str {
    match ret {
        parser::ReturnType::Void => "void",
        parser::ReturnType::Type(parser::Type::String) => "RustString",
        parser::ReturnType::Type(ty) => translate_type(ty)
    }
}

pub fn translate_type(ty: parser::Type) -> &'static str {
    match ty {
        parser::Type::U32 => "unsigned int",
        parser::Type::U16 => "unsigned short",
        parser::Type::U8 => "unsigned char",
        parser::Type::I32 => "int",
        parser::Type::I16 => "short",
        parser::Type::I8 => "char",
        parser::Type::F32 => "float",
        parser::Type::F64 => "double",
        parser::Type::Boolean => "bool",
	    parser::Type::String => "const char*",
        parser::Type::StringRef => "const char*",
        parser::Type::StrRef => "const char*"
    }
}