// File Operation Module
// LER0ever

use std::path::Path;
use std::string::String;

pub fn isDirExist(FolderPath: &str) -> bool {
	let folder = Path::new(FolderPath);
    if folder.exists() == true && folder.is_dir() == true {
        return true;
    }
    return false;
}
