// libksl: Library to KickStart Linux
extern crate winreg;

pub mod lx;
pub mod fs;

#[cfg(target_os = "windows")]
pub mod reg;