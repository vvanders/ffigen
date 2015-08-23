extern crate syntex_syntax;
extern crate toml;

mod parser;
mod csharp;

use std::env;
use std::fs;
use std::path::Path;
use std::fs::File;
use std::io::Read;

use toml::Value;

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
    let root = Path::new(&context.crate_root);
    let (package_name, is_dynamic) = parse_toml(&root);

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
            Lang::CSharp => csharp::gen(&exports, &out_dir),
            _ => ()
        }
    }
}

fn parse_toml(root: &Path) -> (String, bool) {
    let toml_path = root.join("Cargo.toml");

    match fs::metadata(&toml_path) {
        Err(e) => panic!("Unable to open Cargo.toml at {:?} {}", &toml_path, e),
        Ok(meta) => {
            if !meta.is_file() {
                panic!("Cargo.toml at {:?} isn't a file!", &toml_path);
            }
        }
    }

    let mut config_file = match File::open(&toml_path) {
        Err(e) => panic!("Unable to open {:?} {}", &toml_path, e),
        Ok(v) => v
    };

    let mut config_content = String::new();
    if let Err(e) = config_file.read_to_string(&mut config_content) {
        panic!("Unable to read {:?} {}", &toml_path, e);
    }

    let config = match toml::Parser::new(&config_content).parse() {
        None => panic!("toml {:?} was empty", &toml_path),
        Some(v) => Value::Table(v)
    };

    let package_name = match config.lookup("package.name") {
        None => panic!("toml {:?} doesn't have a [package].name", &toml_path),
        Some(v) => {
            match v {
                &Value::String(ref name) => {
                    println!("Found package name {}", v);
                    name.clone()
                },
                _ => panic!("unexpected type for name in toml {:?}", &toml_path)
            }
        }
    };

    let is_dynamic = match config.lookup("lib.crate_type") {
        None => panic!("toml {:?} doesn't have a [lib].crate_type", &toml_path),
        Some(v) => {
            match v {
                &Value::Array(ref arr) => {
                    arr.contains(&Value::String("dylib".to_string()))
                }
                _ => false
            }
        }
    };

    (package_name, is_dynamic)
}