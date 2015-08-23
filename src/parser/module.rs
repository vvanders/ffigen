﻿use std::path::Path;
use std::fs;

use parser::{ModuleDecl, FuncDecl};

pub fn parse(src: &Path, modules: &Vec<ModuleDecl>) -> (Vec<FuncDecl>, Vec<ModuleDecl>) {
    let exports = Vec::new();
    let exports_mod = Vec::new();

    for module in modules {
        let root = match src.parent() {
            Some(v) => v,
            None => panic!("Unable to find root for {:?}", &src)
        };

        let modname = format!("{}.rs", &module.name);
        let filemod = root.join(&modname);

        if let Ok(meta) = fs::metadata(&filemod) {
            if meta.is_file() {
                continue;
            }
        }

        let dirmod = root.join(&module.name).join("mod.rs");

        if let Ok(meta) = fs::metadata(&dirmod) {
            if meta.is_file() {
                continue;
            }
        }

        panic!("Unable to find module {} at {:?} or {:?}", &module.name, &filemod, &dirmod);
    }

    (exports, exports_mod)
}