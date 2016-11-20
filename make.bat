@echo off
mingw32-make -f Makefile.win
rem call "C:\Program Files (x86)\Microsoft Visual Studio 14.0\VC\vcvarsall.bat" x86_amd64
rem set OPENSSL_INCLUDE_DIR=%~dp0/openssl/include64
rem set OPENSSL_LIB_DIR=%~dp0/openssl/lib64
rem set INCLUDE=%INCLUDE%;%~dp0/openssl/include64
rem set LIBPATH=%LIBPATH%;%~dp0/openssl/lib64
rem cargo build --manifest-path libksl\Cargo.toml --release
rem cargo build --manifest-path app\WSAOSC\Cargo.toml --release
rem cargo build --manifest-path ksl_tests\Cargo.toml --release
