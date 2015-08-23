mod module;
mod source;

use std::fs;
use std::path::Path;

#[derive(Copy, Clone)]
pub enum Type {
    U32,
    U16,
    U8,
    I32,
    I16,
    I8,
    F32,
    F64,
    Boolean
}

#[derive(Copy, Clone)]
pub enum ReturnType {
    Void,
    Type(Type)
}

pub struct Arg {
    pub name: String,
    pub ty: Type
}

pub struct FuncDecl {
    pub name: String,
    pub ret: ReturnType,
    pub args: Vec<Arg>
}

pub struct ModuleDecl {
    pub name: String
}

pub fn parse(path: &Path) -> (Vec<FuncDecl>, Vec<ModuleDecl>) {
    match fs::metadata(&path) {
        Err(e) => panic!("Unable to parse {:?} {}", &path, e),
        Ok(_) => ()
    }

    let (mut exports, mut modules) = source::parse(path);
    let (sub_exports, sub_modules) = module::parse(&path, &modules);

    exports.extend(sub_exports.into_iter());
    modules.extend(sub_modules.into_iter());

    (exports, modules)
}