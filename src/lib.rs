extern crate syntex_syntax;

mod parser;
mod csharp;

use std::env;
use std::path::Path;

pub enum Lang {
    CSharp,
    Java,
    C,
    CPP
}

pub fn gen_cargo() {
    let cargo_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let src_dir = Path::new(&cargo_dir).join("src");

    let out_dir = Path::new(&cargo_dir).join("gen");

    let mut langs: Vec<Lang> = Vec::new();
    langs.push(Lang::CSharp);

    gen(&src_dir, &out_dir, &langs);
}

pub fn gen(src: &Path, dest: &Path, langs: &Vec<Lang>) {
    let exports = match parser::parse_dir(src) {
        Ok(v) => v,
        Err(e) => panic!("Unable to export {:?}", e)
    };

    for lang in langs {
        match *lang {
            Lang::CSharp => csharp::gen(&exports, dest),
            _ => ()
        }
    }
}