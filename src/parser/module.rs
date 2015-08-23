use std::path::Path;
use std::fs;

use parser::source;

use parser::{ModuleDecl, FuncDecl};

pub fn parse(src: &Path, modules: &Vec<ModuleDecl>) -> (Vec<FuncDecl>, Vec<ModuleDecl>) {
    let mut exports = Vec::new();
    let mut exports_mod = Vec::new();

    for module in modules {
        let root = match src.parent() {
            Some(v) => v,
            None => panic!("Unable to find root for {:?}", &src)
        };

        let modname = format!("{}.rs", &module.name);
        let filemod = root.join(&modname);

        if let Ok(meta) = fs::metadata(&filemod) {
            if meta.is_file() {
                join_exports(&filemod, &mut exports, &mut exports_mod);
                continue;
            }
        }

        let dirmod = root.join(&module.name).join("mod.rs");

        if let Ok(meta) = fs::metadata(&dirmod) {
            if meta.is_file() {
                join_exports(&dirmod, &mut exports, &mut exports_mod);
                continue;
            }
        }

        panic!("Unable to find module {} at {:?} or {:?}", &module.name, &filemod, &dirmod);
    }

    (exports, exports_mod)
}

fn join_exports(src: &Path, exports: &mut Vec<FuncDecl>, modules: &mut Vec<ModuleDecl>) {
    let (new_exports, new_modules) = source::parse(src);

    exports.extend(new_exports.into_iter());
    modules.extend(new_modules.into_iter());
}