use std::path::Path;
use std::fs;
use std::fs::File;
use std::io::Read;

use toml;
use toml::Value;

pub struct Info {
    pub name: String,
    pub lib_name: String,
    pub is_dynamic: bool
}

pub fn parse(root: &Path) -> Info {
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
                    println!("Found package {}", v);
                    name.clone()
                },
                _ => panic!("unexpected type for name in toml {:?}", &toml_path)
            }
        }
    };

    let lib_name = match config.lookup("lib.name") {
        None => package_name.clone(), //No prop defined, we'll fallback to package name
        Some(v) => {
            match v {
                &Value::String(ref name) => {
                    name.clone()
                },
                _ => panic!("unexpected type for library name in tomp {:?}", &toml_path)
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

    Info { name: package_name, lib_name: lib_name, is_dynamic: is_dynamic }
}