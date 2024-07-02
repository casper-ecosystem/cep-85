ALL_CONTRACTS = cep85 cep85-test-contract
TARGET_DIR = $(CURDIR)/target
CONTRACT_TARGET_DIR = $(TARGET_DIR)/wasm32-unknown-unknown/release
PINNED_TOOLCHAIN := $(shell cat contracts/rust-toolchain)

export CARGO_TARGET_DIR=$(TARGET_DIR)

prepare:
	rustup target add wasm32-unknown-unknown
	rustup component add clippy --toolchain ${PINNED_TOOLCHAIN}
	rustup component add rustfmt --toolchain ${PINNED_TOOLCHAIN}
	rustup component add rust-src --toolchain ${PINNED_TOOLCHAIN}

.PHONY:	build-contract
build-contract:
	cd contracts/cep85 && RUSTFLAGS="-C target-cpu=mvp" cargo build --release --target wasm32-unknown-unknown -Z build-std=std,panic_abort
	wasm-strip $(CONTRACT_TARGET_DIR)/cep85.wasm

.PHONY:	build-all-contracts
build-all-contracts:
	cd contracts && RUSTFLAGS="-C target-cpu=mvp" cargo build --release --target wasm32-unknown-unknown $(patsubst %,-p %, $(ALL_CONTRACTS)) -Z build-std=std,panic_abort
	$(foreach WASM, $(ALL_CONTRACTS), wasm-strip $(CONTRACT_TARGET_DIR)/$(subst -,_,$(WASM)).wasm ;)
	cd client/make_dictionary_item_key && RUSTFLAGS="-C target-cpu=mvp" cargo build --release --target wasm32-unknown-unknown -Z build-std=std,panic_abort
	wasm-strip $(CONTRACT_TARGET_DIR)/cep85_make_dictionary_item_key.wasm

setup-test: build-all-contracts
	mkdir -p tests/wasm
	cp $(CONTRACT_TARGET_DIR)/cep85.wasm tests/wasm
	cp $(CONTRACT_TARGET_DIR)/cep85_test_contract.wasm tests/wasm
	cp $(CONTRACT_TARGET_DIR)/cep85_make_dictionary_item_key.wasm tests/wasm

test: setup-test
	cd tests && cargo test

clippy:
	cd contracts && cargo clippy --bins -- -D warnings
	cd contracts && cargo clippy --lib -- -D warnings
	cd contracts && cargo clippy --lib --no-default-features -- -D warnings
	cd tests && cargo clippy --all-targets -- -D warnings

check-lint: clippy
	cd contracts && cargo fmt -- --check
	cd tests && cargo fmt -- --check

format:
	cd contracts && cargo fmt
	cd tests && cargo fmt

clean:
	cd contracts && cargo clean
	cd tests && cargo clean
	rm -rf tests/wasm
