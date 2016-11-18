extern crate libksl;

#[test]
fn TESTisDirExist() {
	assert_eq!(libksl::fs::isDirExist("src"), true);
}
