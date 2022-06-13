all: check

check:
	cargo fmt --check
	cargo clippy

test:
	cargo test
