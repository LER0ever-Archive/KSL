pub enum OSType {
    Linux,
    macOS,
    Windows,
    FreeBSD,
    VoidOS
}

pub enum Arch {
    amd64,
    i386,
    powerpc64,
    arm,
    aarch64
}

pub fn get_os_type() -> OSType {
    if cfg!(target_os = "windows"){
        return OSType::Windows;
    }
    if cfg!(target_os = "linux"){
        return OSType::Linux;
    }
    if cfg!(target_os = "macos"){
        return OSType::macOS;
    }
    if cfg!(target_os = "freebsd"){
        return OSType::FreeBSD;
    }
    OSType::Linux
}

pub fn get_arch() -> Arch {
    if cfg!(target_arch = "x86_64"){
        return Arch::amd64;
    }
    if cfg!(target_arch = "x86") {
        return Arch::i386;
    }
    if cfg!(target_arch = "powerpc64"){
        return Arch::powerpc64;
    }
    if cfg!(target_arch = "arm"){
        return Arch::arm;
    }
    if cfg!(target_arch = "aarch64") {
        return Arch::aarch64;
    }
    Arch::amd64
}
