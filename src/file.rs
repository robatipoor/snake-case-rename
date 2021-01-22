use std::fs::DirEntry;

pub fn create_name_file(name: &str, ext: Option<String>, is_hiden: bool) -> String {
    let mut new_name = String::new();
    if is_hiden {
        new_name.push('.');
    }
    new_name.push_str(name);
    if let Some(ex) = ext {
        new_name.push('.');
        new_name.push_str(&ex);
    }
    new_name
}

pub fn get_extension_file(dir: &DirEntry) -> Option<String> {
    let ext = dir.path().extension().to_owned()?.to_str()?.to_string();
    Some(ext)
}

pub fn get_name(dir: &DirEntry) -> Option<String> {
    let name = dir
        .path()
        .file_stem()
        .to_owned()?
        .to_owned()
        .to_str()?
        .to_string();
    Some(name)
}

pub fn check_is_hiden(name: &str) -> bool {
    if name.len() > 0 {
        name.as_bytes()[0] as char == '.'
    } else {
        false
    }
}
