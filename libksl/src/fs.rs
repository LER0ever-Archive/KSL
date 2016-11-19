// File Operation Module
// LER0ever

use std::path::Path;
use std::string::String;

pub fn is_dir_exist(FolderPath: &str) -> bool {
	let folder = Path::new(FolderPath);
    if folder.exists() == true && folder.is_dir() == true {
        return true;
    }
    return false;
}

pub fn is_file_exist(FilePath: &str) -> bool {
    let file = Path::new(FilePath);
    if file.exists() == true && file.is_file() == true {
        return true;
    }
    return false;
}