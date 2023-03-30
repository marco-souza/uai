all: build test

dev: src/main.rs
	cargo run "hello world"

build: src/main.rs
	cargo build --release

install: src/main.rs
	cargo install --path .

test: src/main.rs
	cargo test

publish: src/main.rs Cargo.toml
	cargo publish

fmt: src/main.rs
	rustfmt src/* --edition 2021
