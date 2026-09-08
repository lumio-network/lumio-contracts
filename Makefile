# Lumio Contracts — build / test / deploy shortcuts.
# Run `make help` for the list.

.PHONY: help build test fmt fmt-check wasm clean

help:
	@echo "build      - cargo build --workspace"
	@echo "test       - cargo test --workspace"
	@echo "fmt        - format all crates"
	@echo "fmt-check  - verify formatting (CI)"
	@echo "wasm       - build optimized wasm for all contracts"
	@echo "clean      - cargo clean"

build:
	cargo build --workspace

test:
	cargo test --workspace

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

wasm:
	cargo build --workspace --target wasm32v1-none --release

clean:
	cargo clean
