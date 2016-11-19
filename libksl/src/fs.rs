// File Operation Module
// LER0ever

use std::path::Path;

pub fn is_dir_exist(folder_path: &str) -> bool {
	let folder = Path::new(folder_path);
    if folder.exists() == true && folder.is_dir() == true {
        return true;
    }
    return false;
}

pub fn is_file_exist(file_path: &str) -> bool {
    let file = Path::new(file_path);
    if file.exists() == true && file.is_file() == true {
        return true;
    }
    return false;
}
