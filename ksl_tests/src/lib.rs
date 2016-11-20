extern crate libksl;
#[macro_use] extern crate matches;

pub mod fs_test;
pub mod sys_test;

#[cfg(target_os = "windows")]
pub mod reg_test;