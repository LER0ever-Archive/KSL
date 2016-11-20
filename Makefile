# I don't know how to write makefile \o/
all: libksl WSAOSC
libksl: libksl/*
	cargo build --manifest-path ./libksl/Cargo.toml --release
tests: ksl_tests/*
	cargo test --manifest-path ./ksl_tests/Cargo.toml --release
WSAOSC: app/WSAOSC/*
	cargo build --manifest-path ./app/WSAOSC/Cargo.toml --release