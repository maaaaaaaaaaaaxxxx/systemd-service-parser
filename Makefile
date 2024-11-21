fmt:
	cargo format

clippy:
	cargo clippy -- -D warnings

test:
	cargo test

build:
	cargo build

run:
	cargo run -- test.service

all: fmt clippy test build
