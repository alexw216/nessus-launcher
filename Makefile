ROOT := $(CURDIR)
CLI := $(ROOT)/nessus-cli

.PHONY: build run release clean fmt check

build:
	@echo "==> Building library"
	cargo build --manifest-path $(ROOT)/Cargo.toml
	@echo "==> Building CLI"
	cargo build --manifest-path $(CLI)/Cargo.toml

run:
	@echo "==> Running CLI"
	cargo run --manifest-path $(CLI)/Cargo.toml -- $(ARGS)

release:
	@echo "==> Building release binaries"
	cargo build --release --manifest-path $(ROOT)/Cargo.toml
	cargo build --release --manifest-path $(CLI)/Cargo.toml

clean:
	cargo clean

fmt:
	cargo fmt --all

check:
	cargo check --all

