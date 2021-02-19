pub mod arg;
pub mod file;
pub mod punct;
pub mod result;

use crate::file::*;
use crate::punct::*;
use crate::result::Result;
use arg::*;
use std::fs;

fn main() {
    if let Err(e) = run() {
        eprintln!("error => {}", e);
    }
}

fn run() -> crate::Result {
    let args = get_args();
    let dirs = read_current_dir()?;
    let mut dup = Vec::<String>::new();
    let mut counter = 1;
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
        let mut new_name = create_name_file(&name, ext, is_hiden);
        let mut new_name = snake_case_convert(new_name);
        if dup.contains(&new_name) {
            new_name = format!("{:02}_{}", counter, new_name);
            counter += 1;
        }
        if args.apply {
            fs::rename(dir.path().file_name().unwrap(), &new_name).unwrap();
            println!(
                "{} rename to => {}  ",
                dir.path().file_name().unwrap().to_str().unwrap(),
                new_name
            );
        } else {
            print!("{}  ", new_name);
        }
        dup.push(new_name);
    }
    println!();
    Ok(())
}
