use std::path::Path;
use std::io;
use winreg::RegKey;
use winreg::enums::*;
use std::string::String;

pub fn get_int_value(path_from_lm: &str, key_name: &str) -> u32 {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let subkey = hklm.open_subkey_with_flags(path_from_lm, KEY_READ).unwrap();
    let int_value: u32 = subkey.get_value(key_name).unwrap();
    // assert_eq!(typeof(int_value), u32);
    return int_value;
}

pub fn get_str_value(path_from_lm: &str, key_name: &str) -> String {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let subkey = hklm.open_subkey_with_flags(path_from_lm, KEY_READ).unwrap();
    let str_value: String = subkey.get_value(key_name).unwrap();
    // assert_eq!(typeof(str_value), &str);
    str_value
}
