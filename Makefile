.PHONY: build test format

build:
	@cargo build

test:
	@cargo test

format:
	@cargo fmt -- --emit files
