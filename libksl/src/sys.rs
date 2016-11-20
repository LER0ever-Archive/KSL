pub enum OSType {
    Windows,
    Linux,
    macOS,
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

#[macro_export]
macro_rules! match_cfg {
    ( $( ($cfg:meta) => $e:expr, )* _ => $last:expr, ) => {
        match () {
            $(
                #[cfg($cfg)]
                () => $e,
            )*

            #[cfg(all( $( not($cfg) ),* ))]
            () => $last,
        }
    };

    ( $( ($cfg:meta) => $e:expr, )* ) => {
        match_cfg! {
            $(
                ($cfg) => $e,
            )*

            _ => {
                #[allow(dead_code)]
                #[static_assert]
                static MATCH_CFG_FALLBACK_UNREACHABLE: bool = false;
            },
        }
    };
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
}

pub fn get_arch() -> Arch {
    match_cfg! {
        (target_arch = "x86_64") => Arch::amd64,
        (target_arch = "x86") => Arch::i386,
        (target_arch = "powerpc64") => Arch::powerpc64,
        (target_arch = "arm") => Arch::arm,
        (target_arch = "aarch64") => Arch::aarch64,
    }
}
