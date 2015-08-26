extern crate syntex_syntax;
extern crate toml;

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

#[derive(Clone)]
pub enum Config {
    Output(String)
}

pub struct Context {
    crate_root: String,
    langs: Vec<(Lang, Vec<Config>)>
}

pub fn new_context() -> Context {
    Context { crate_root: env::var("CARGO_MANIFEST_DIR").unwrap(), langs: Vec::new() }
}

impl Context {
    pub fn add_lang(&mut self, lang: Lang, opts: &[Config]) {
        let mut vec_opts: Vec<Config> = Vec::new();

        for opt in opts {
            let opt_value = opt.clone();
            vec_opts.push(opt_value);
        }

        self.langs.push((lang, vec_opts));
    }
}

pub fn gen_cargo() {
    let mut context = new_context();

    context.langs.push((Lang::CSharp, Vec::new()));

    gen(&context);
}

pub fn gen(context: &Context) {
    let root = Path::new(&context.crate_root);
    let package_info = parser::cargo::parse(&root);

    let src_dir = root.join("src").join("lib.rs");
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
            Lang::CSharp => csharp::gen(&exports, &package_info, &out_dir),
            _ => ()
        }
    }
}