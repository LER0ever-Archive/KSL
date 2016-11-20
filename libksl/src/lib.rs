// libksl: Library to KickStart Linux
#[cfg(target_os = "windows")]
extern crate winreg;

pub mod lx;
pub mod fs;

#[cfg(target_os = "windows")]
pub mod reg;
