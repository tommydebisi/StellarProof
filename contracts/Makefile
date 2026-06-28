.PHONY: test build

TARGET := wasm32v1-none

test:
	cd .. && cargo test --workspace

build:
	cd .. && cargo build --manifest-path contracts/provenance/Cargo.toml --target $(TARGET) --release
	cd .. && cargo build --manifest-path contracts/stellarproof/Cargo.toml --target $(TARGET) --release
	cd .. && cargo build --manifest-path contracts/registry/Cargo.toml --target $(TARGET) --release
	cd .. && cargo build --manifest-path contracts/oracle/Cargo.toml --target $(TARGET) --release
