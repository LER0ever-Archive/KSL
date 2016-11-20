use libksl::sys::*;

#[test]
pub fn test_get_os_type(){
    #[cfg(target_os = "windows")]
    assert!(matches!(get_os_type(), OSType::Windows));
}

#[test]
pub fn test_get_arch() {
    #[cfg(target_arch = "amd64")]
    assert!(matches!(get_arch(), Arch::amd64));
}