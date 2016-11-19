use libksl::reg::get_str_value;

#[test]
fn test_get_int_value() {
}

#[test]
fn test_get_str_value() {
    let OS_NT = get_str_value("SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment", "OS");
    let str_OS_NT = &OS_NT;
    assert_eq!(str_OS_NT, "Windows_NT");
}