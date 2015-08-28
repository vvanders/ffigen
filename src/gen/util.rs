use std::path::Path;

use std::ops::Deref;

use std;
use std::fs;
use std::io;
use std::io::Write;

pub fn write_source(content: &String, out_path: &Path) -> std::io::Result<()> {
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