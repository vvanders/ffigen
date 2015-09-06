use std::path;

use std::ops::Deref;

use std;
use std::fs;
use std::io;
use std::io::Write;

pub fn write_source(content: &String, out_path: &path::Path) -> std::io::Result<()> {
    let folder = out_path.parent().unwrap();
    if let Err(metae) = fs::metadata(folder) {
        if metae.kind() == io::ErrorKind::NotFound {
            println!("Creating {:?}", folder);
            if let Err(e) = fs::create_dir(folder) {
                return Err(e);
            }
        }
    }

    let file = match fs::File::create(&out_path) {
        Ok(f) => f,
        Err(e) => return Err(e)
    };

    let mut writer = io::BufWriter::new(&file);
    let bytes = content.clone().into_bytes();

    writer.write_all(bytes.into_boxed_slice().deref())
}

pub fn get_namespace(opts: &Vec<::Config>, package_name: &String) -> String {
    let opt_ns = opts.iter()
        .filter_map(|o| match *o {
            ::Config::Namespace(ref ns) => Some(ns.clone()),
            _ => None
        })
        .fold(None, |_, o| Some(o.clone()));

    match opt_ns {
        Some(s) => s,
        None => package_name.clone()
    }
}

pub fn get_output_dir(opts: &Vec<::Config>, crate_root: &path::PathBuf) -> path::PathBuf {
    //Select our option if we have it
    let opt_dir = opts.iter()
        .filter_map(|o| match *o {
            ::Config::Output(ref s) => Some(s.clone()),
            _ => None
        })
        .fold(None, |_, o| Some(o.clone()));

    match opt_dir {
        Some(ref s) => path::Path::new(s).to_path_buf(),
        None => crate_root.join("gen")
    }
}