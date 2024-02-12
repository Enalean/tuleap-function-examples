SHELL=/usr/bin/env bash -euo pipefail -c

.PHONY: prepare
prepare:
	true

.PHONY: tests
tests:
	cargo test

.PHONY: build
build:
	cargo build --target wasm32-wasi --release
