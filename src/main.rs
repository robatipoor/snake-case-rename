pub mod arg;
pub mod file;
pub mod punct;
pub mod result;

use arg::*;
use crate::file::*;
use crate::punct::*;
use crate::result::Result;
use std::fs;

fn main() {
    if let Err(e) = run() {
        eprintln!("error => {}", e);
    }
}

fn run() -> crate::Result {
    let args = get_args();
    let dirs = read_current_dir()?;
    for dir in dirs.iter() {
        let name = match get_name(dir) {
            Some(n) => n,
            None => {
                continue;
            }
        };
        let ext = get_extension_file(dir);
        let is_hiden = check_is_hiden(&name);
        let name = replace_punct_with_underscore(&name);
        let new_name = create_name_file(&name, ext, is_hiden).to_lowercase();
        if args.apply {
            fs::rename(dir.path().file_name().unwrap(), new_name).unwrap();
        } else {
            print!("{}  ", new_name);
        }
    }
    Ok(())
}
