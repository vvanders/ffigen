extern crate syntex_syntax;
extern crate toml;

mod parser;
mod gen;
pub mod marshal;

use std::env;
use std::path::Path;

pub enum Lang {
    CSharp,
    Java,
    C,
    Cpp
}

#[derive(Clone)]
pub enum Config {
    Output(String),
    Namespace(String)
}

pub struct Context {
    crate_root: String,
    output_wrapper: String,
    langs: Vec<(Lang, Vec<Config>)>
}

impl Context {
    pub fn new() -> Context {
        let mut context = Context { crate_root: "".to_string(), output_wrapper: "".to_string(), langs: Vec::new() };

        context.set_root(env::var("CARGO_MANIFEST_DIR").unwrap());

        context
    }

    pub fn set_root(&mut self, path: String) {
        let src = Path::new(&path).join("src");

        self.crate_root = path;
        self.output_wrapper = String::from(src.to_str().unwrap());
    }

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
    let mut context = Context::new();

    context.langs.push((Lang::CSharp, Vec::new()));

    gen(&context);
}

pub fn gen(context: &Context) {
    let root = Path::new(&context.crate_root);
    let package_info = parser::cargo::parse(&root);

    let src_dir = root.join("src").join("lib.rs");
    
    println!("Parsing source at {:?}", &src_dir);

    let (exports, _) = parser::parse(&src_dir, &"".to_string());

    let marshal_result = gen::marshal::gen(&exports, Path::new(&context.output_wrapper));

    if let Err(e) = marshal_result {
        panic!("Unable to export {}", e);
    }
    
    for &(ref lang, ref opts) in context.langs.iter() {
        let result = match *lang {
            Lang::CSharp => gen::csharp::gen(&exports, &package_info, &opts),
            Lang::Cpp => gen::cpp::gen(&exports, &package_info, &opts),
            Lang::C => gen::c::gen(&exports, &package_info, &opts),
            Lang::Java => Ok(())
        };

        if let Err(e) = result {
            panic!("Unable to export {}", e);
        }
    }
}