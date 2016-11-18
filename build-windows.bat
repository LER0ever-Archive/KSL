@echo off
call "C:\Program Files (x86)\Microsoft Visual Studio 14.0\VC\vcvarsall.bat" x86_amd64
set OPENSSL_INCLUDE_DIR=%~dp0/openssl/include64
set OPENSSL_LIB_DIR=%~dp0/openssl/lib64
set INCLUDE=%INCLUDE%;%~dp0/openssl/include64
set LIBPATH=%LIBPATH%;%~dp0/openssl/lib64
cargo build --release