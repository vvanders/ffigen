use std;

use parser;
use parser::cargo;

use gen::util;
use gen::commonc;

pub fn gen(exports: &Vec<parser::FuncDecl>, package_info: &cargo::Info, opts: &Vec<::Config>) -> std::io::Result<()> {
    let namespace = util::get_namespace(opts, &package_info.name);

    let static_only = opts.iter().any(|opt| match *opt { ::Config::StaticOnly => true, _ => false });
    let dynamic_only = opts.iter().any(|opt| match *opt { ::Config::DynamicOnly => true, _ => false }); 

    let dynamic = if static_only {
        if !package_info.is_static {
            panic!("Cannot export library statically, doesn't provide static export");
        }

        false
    } else if dynamic_only {
        if !package_info.is_dynamic {
            panic!("Cannot export library dynamically, doesn't provide dynamic export");
        }

        true
    } else {
        false
    };

    let mut header = start_header(&namespace, &package_info.name);
    header.push_str(export_functions(exports, !dynamic).as_ref());
    header.push_str(end_header(&namespace, &package_info.name).as_ref());
    header = commonc::indent(header);

    let header_path = util::get_output_dir(opts, &package_info.crate_root).join(format!("{}.h", package_info.name));
    if let Err(e) = util::write_source(&header, &header_path) {
        return Err(e);
    }

    if dynamic {
        let mut source = start_source(&namespace, &package_info.lib_name, &package_info.name);
        source.push_str(generate_functions(exports).as_ref());
        source.push_str(end_source(&namespace).as_ref());
        source = commonc::indent(source);

        let source_path = util::get_output_dir(opts, &package_info.crate_root).join(format!("{}.cpp", package_info.name));
        if let Err(e) = util::write_source(&source, &source_path) {
            return Err(e);
        }
    }

    Ok(())
}

fn start_header(namespace: &String, package_name: &String) -> String {
    let mut content = commonc::get_header_guard_start(package_name);
    
    content.push_str(commonc::get_namespace_start(namespace).as_ref());
    content.push_str(commonc::get_string_decl().as_ref());
    content.push_str("\n");

    content
}

fn end_header(namespace: &String, package_name: &String) -> String {
    let mut content = String::new();

    content.push_str(commonc::get_namespace_end(namespace).as_ref());
    content.push_str(commonc::get_header_guard_end(package_name).as_ref());

    content
}

fn get_function_decl(func: &parser::FuncDecl) -> String {
    let params = func.args.iter()
        .map(|arg| commonc::translate_type(arg.ty).to_string() + " " + arg.name.as_ref() )
        .fold(String::new(), |acc, arg| match acc.len() {
            0 => arg,
            _ => acc + ", " + arg.as_ref()
        });

    format!("{} {}({})", commonc::translate_return_type(func.ret), func.name, params)
}

fn export_functions(exports: &Vec<parser::FuncDecl>, isStatic: bool) -> String {
    let mut content = exports.iter().
        map(|func| get_function_decl(func) + ";")
        .fold(String::new(), |acc, arg| match acc.len() {
            0 => arg,
            _ => acc + "\n" + arg.as_ref()
        });
    
    content.push_str("\n");
    content.push_str("\n");
    content.push_str(commonc::get_string_funcs().as_ref());
    content.push_str("\n");

    //If we've got a static library then we just need to mark this as external and we're done
    content.split("\n")
        .map(|line| {
            if isStatic && line.len() > 0 {
                "extern \"C\" ".to_string() + line
            } else {
                line.to_string()
            }
        })
        .fold(String::new(), |acc, line| match acc.len() {
            0 => line,
            _ => acc + "\n" + line.as_ref()
        })
}

fn start_source(namespace: &String, library_name: &String, crate_name: &String) -> String {
    let mut content = String::new();
    
    content.push_str(format!("#include \"{}.h\"\n\n", crate_name).as_ref());

    content.push_str(r"#ifdef WIN32
    #define WIN32_LEAN_AND_MEAN
    #include <windows.h>
#else
    #include <dlfcn.h>
#endif  //WIN32

");

    content.push_str(commonc::get_namespace_start(namespace).as_ref());

    let funcLoader = format!(r#"void* GetAddr(const char* name) {{
#ifdef WIN32    
    static HMODULE dllHandle = LoadLibrary(L"{}.dll");
    return GetProcAddress(dllHandle, name);
#else
    static void* soHandle = dlopen("{}.so", RTLD_LAZY);
    return dlsym(soHandle, name);
#endif  //WIN32
}}

"#, library_name, library_name);

    content.push_str(funcLoader.as_ref());

    content
}

fn end_source(namespace: &String) -> String {
    let mut content = String::new();

    content.push_str(commonc::get_namespace_end(namespace).as_ref());

    content
}

fn generate_functions(exports: &Vec<parser::FuncDecl>) -> String {
    let content = exports.iter()
        .map(|func| {
            let args = func.args.iter().fold(String::new(), |acc, arg| match acc.len() {
                0 => arg.name.clone(),
                _ => acc + ", " + arg.name.as_ref()
            });

            let call = match func.ret {
                parser::ReturnType::Void => format!("funcPtr({});", args),
                parser::ReturnType::Type(_) => format!("return funcPtr({});", args)
            };

            let typedef = format!("typedef {} (*FuncSignature)({});",
                commonc::translate_return_type(func.ret),
                func.args.iter()
                    .map(|arg| commonc::translate_type(arg.ty))
                    .fold(String::new(), |acc, arg| {
                        match acc.len() {
                            0 => arg.to_string(),
                            _ => acc + ", " + arg.as_ref()
                        }
                    })
                );

            format!(
r#"{} {{
{}
static FuncSignature funcPtr = reinterpret_cast<FuncSignature>(GetAddr("{}"));
{}
}}
"#, get_function_decl(func), typedef, func.name, call)
        })
        .fold(String::new(), |acc, func| match acc.len() {
            0 => func,
            _ => acc + "\n" + func.as_ref()
        });

    content
}
