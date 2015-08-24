use std::ops::Deref;

use std::path::Path;

use parser::{Type, ReturnType, ModuleDecl, FuncDecl, Arg};

use syntex_syntax::ast;
use syntex_syntax::abi;
use syntex_syntax::parse;
use syntex_syntax::ptr::P;
use syntex_syntax::visit;
use syntex_syntax::print;

struct FnVisitor<'a> {
    exports: &'a mut Vec<FuncDecl>,
    modules: &'a mut Vec<ModuleDecl>
}

impl<'a> FnVisitor<'a> {
    fn new(exports: &'a mut Vec<FuncDecl>, modules: &'a mut Vec<ModuleDecl>) -> FnVisitor<'a> {
        FnVisitor { exports: exports, modules: modules }
    }
}

impl<'v> visit::Visitor<'v> for FnVisitor<'v> {
    fn visit_item(&mut self, item: &'v ast::Item) {
        match item.node {
            ast::ItemFn(ref decl, _, _, abi, _, _) if abi == abi::C => {
                let mangle = item.attrs.iter()
                        .any(|v| {
                            match v.node.value.node {
                                ast::MetaItem_::MetaWord(ref w) => interned_to_string(w) == "no_mangle",
                                _ => false
                            }
                        });

                //Don't export private functions
                if item.vis == ast::Visibility::Public {
                    if mangle {
                        let export = sanitize_export(&item.ident.name, &decl.output, &decl.inputs);

                        println!("Exporting {}", &export.name);
                        self.exports.push(export);
                    } else {
                    }
                } else {
                    println!("Skipping export of {} due to not being public", &name_to_string(&item.ident.name));
                }
            },
            ast::ItemMod(_) => {
                if item.vis == ast::Visibility::Public {
                    let export = sanitize_module(&item.ident.name);
                    println!("Exporting module {}", export.name);

                    self.modules.push(export);
                } else {
                    println!("Skipping export of module {} due to not being public", &name_to_string(&item.ident.name));
                }
            },
            _ => ()
        }
    }
}

pub fn parse(path: &Path) -> (Vec<FuncDecl>, Vec<ModuleDecl>) {
    let mut exports: Vec<FuncDecl> = Vec::new();
    let mut modules: Vec<ModuleDecl> = Vec::new();

    let cfg: Vec<P<ast::MetaItem>> = Vec::new();
    let sess = parse::ParseSess::new();
    let krate = parse::parse_crate_from_file(path, cfg, &sess);

    //Scope exports and modules so we can return
    {
        let mut visitor = FnVisitor::new(&mut exports, &mut modules);
        visit::walk_crate(&mut visitor, &krate);
    }

    (exports, modules)
}

fn sanitize_module(name: &ast::Name) -> ModuleDecl {
    let module_name = name.deref().as_str().deref().to_string();
    ModuleDecl { name: module_name }
}

fn sanitize_export(name: &ast::Name, ret: &ast::FunctionRetTy, args: &Vec<ast::Arg>) -> FuncDecl {
    let ret_san = match *ret {
        ast::FunctionRetTy::DefaultReturn(_) | ast::FunctionRetTy::NoReturn(_) => ReturnType::Void,
        ast::FunctionRetTy::Return(ref r) => ReturnType::Type(translate_type(r))
    };

    let mut args_san = Vec::new();

    for arg in args {
        args_san.push(Arg { name: print::pprust::pat_to_string(arg.pat.deref()), ty: translate_type(&arg.ty) }); 
    }

    let func_name = name_to_string(&name);

    FuncDecl { name: func_name, ret: ret_san, args: args_san }
}

fn interned_to_string(interned: &parse::token::InternedString) -> String {
    interned.deref().to_string()
}

fn name_to_string(name: &ast::Name) -> String {
    interned_to_string(&name.deref().as_str())
}

fn translate_type(ty: &P<ast::Ty>) -> Type {
    let rust_type = print::pprust::to_string(|s| s.print_type(ty));

    match rust_type.as_ref() {
        "u32" => Type::U32,
        "u16" => Type::U16,
        "u8" => Type::U8,
        "i32" => Type::I32,
        "i16" => Type::I16,
        "i8" => Type::I8,
        "f32" => Type::F32,
        "f64" => Type::F64,
        "bool" => Type::Boolean,
        _ => panic!("Unknown type {}", rust_type)
    }
}