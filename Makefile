# I don't know how to write makefile \o/
all: env libksl WSAOSC
env: 
	call "C:\Program Files (x86)\Microsoft Visual Studio 14.0\VC\vcvarsall.bat" x86_amd64
	set OPENSSL_INCLUDE_DIR=%~dp0/openssl/include64
	set OPENSSL_LIB_DIR=%~dp0/openssl/lib64
	set INCLUDE=%INCLUDE%;%~dp0/openssl/include64
	set LIBPATH=%LIBPATH%;%~dp0/openssl/lib64
libksl: libksl/*
	cargo build --manifest-path libksl\Cargo.toml --release
tests: ksl_tests/*
	cargo test --manifest-path ksl_tests\Cargo.toml --release
WSAOSC: app/WSAOSC/*
	cargo build --manifest-path app\WSAOSC\Cargo.toml --release