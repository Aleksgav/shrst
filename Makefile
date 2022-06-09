all: check

check:
	cargo fmt --check
	cargo clippy
