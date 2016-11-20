use libksl::fs::*;

#[test]
fn test_is_dir_exist() {
	assert_eq!(is_dir_exist("src"), true);
}

#[test]
fn test_is_file_exist() {
    assert_eq!(is_file_exist("Cargo.toml"), true);
}