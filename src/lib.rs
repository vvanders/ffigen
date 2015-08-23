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

pub enum Config {
    Output(String)
}

pub struct Context {
    crate_root: String,
    langs: Vec<(Lang, Vec<Config>)>
}

pub fn gen_cargo() {
    let cargo_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut context = Context { crate_root: cargo_dir.clone(), langs: Vec::new() };

    context.langs.push((Lang::CSharp, Vec::new()));

    gen(&context);
}

pub fn gen(context: &Context) {
    let src_dir = Path::new(&context.crate_root).join("src").join("lib.rs");
    let (exports, _) = parser::parse(&src_dir);
    
    for &(ref lang, ref opt) in context.langs.iter() {
        //Select our option if we have it
        let opt_dir = opt.iter()
                .filter_map(|o| match *o { Config::Output(ref s) => Some(s.clone()) })
                .fold(None, |_, o| Some(o.clone()));

        let out_dir = match opt_dir {
            Some(ref s) => Path::new(s).to_path_buf(),
            None => {
                Path::new(&context.crate_root).join("gen")
            }
        };

        match *lang {
            Lang::CSharp => csharp::gen(&exports, &out_dir),
            _ => ()
        }
    }
}