PINNED_TOOLCHAIN := $(shell cat contracts/rust-toolchain)
WASM_TARGET_DIR := ./target/wasm32-unknown-unknown/release
WASM_OUTPUT_DIR := tests/wasm
WASM_FILES := cep85.wasm cep85_test_contract.wasm cep85_make_dictionary_item_key.wasm
RUSTFLAGS := -C target-cpu=mvp
CARGO_BUILD_FLAGS := -Z build-std=std,panic_abort

prepare:
	rustup install $(PINNED_TOOLCHAIN)
	rustup target add wasm32-unknown-unknown
	rustup component add clippy --toolchain $(PINNED_TOOLCHAIN)
	rustup component add rustfmt --toolchain $(PINNED_TOOLCHAIN)
	rustup component add rust-src --toolchain $(PINNED_TOOLCHAIN)

.PHONY: build-contract
build-contract:
	RUSTFLAGS="$(RUSTFLAGS)" cargo +$(PINNED_TOOLCHAIN) build --release --target wasm32-unknown-unknown $(CARGO_BUILD_FLAGS) -p cep85
	wasm-strip $(WASM_TARGET_DIR)/$(word 1, $(WASM_FILES))

.PHONY: build-all-contracts
build-all-contracts: build-contract
	RUSTFLAGS="$(RUSTFLAGS)" cargo +$(PINNED_TOOLCHAIN) build --release --target wasm32-unknown-unknown $(CARGO_BUILD_FLAGS) -p cep85-test-contract
	RUSTFLAGS="$(RUSTFLAGS)" cargo +$(PINNED_TOOLCHAIN) build --release --target wasm32-unknown-unknown $(CARGO_BUILD_FLAGS) -p cep85-make-dictionary-item-key
	wasm-strip $(WASM_TARGET_DIR)/$(word 2, $(WASM_FILES))

.PHONY: setup-test
setup-test: build-all-contracts copy-wasm

.PHONY: copy-wasm
copy-wasm:
	mkdir -p $(WASM_OUTPUT_DIR)
	cp $(addprefix $(WASM_TARGET_DIR)/, $(WASM_FILES)) $(WASM_OUTPUT_DIR)

native-test: setup-test
	cargo test -p tests --lib should_transfer_account_to_account

test: setup-test
	cargo test -p tests --lib

clippy:
	cargo +$(PINNED_TOOLCHAIN) clippy --release -p cep85 --bins --target wasm32-unknown-unknown $(CARGO_BUILD_FLAGS) -- -D warnings
	cargo +$(PINNED_TOOLCHAIN) clippy --release -p cep85 --lib --target wasm32-unknown-unknown $(CARGO_BUILD_FLAGS) -- -D warnings
	cargo +$(PINNED_TOOLCHAIN) clippy --release -p cep85 --lib --target wasm32-unknown-unknown $(CARGO_BUILD_FLAGS) --no-default-features -- -D warnings
	cargo +$(PINNED_TOOLCHAIN) clippy -p cep85-test-contract --bins --target wasm32-unknown-unknown $(CARGO_BUILD_FLAGS) -- -D warnings
	cargo clippy -p tests --all-targets -- -D warnings

format:
	cargo +$(PINNED_TOOLCHAIN) fmt -p cep85
	cargo +$(PINNED_TOOLCHAIN) fmt -p cep85-test-contract
	cargo fmt -p tests

check-lint: clippy
	cargo +$(PINNED_TOOLCHAIN) fmt -p cep85
	cargo +$(PINNED_TOOLCHAIN) fmt -p cep85-test-contract
	cargo fmt -p tests -- --check

lint: clippy format

clean:
	cargo clean -p cep85
	cargo clean -p cep85-test-contract
	cargo clean -p tests
	rm -rf $(WASM_OUTPUT_DIR)
	rm -rf ./*/Cargo.lock

.PHONY: cargo-update
cargo-update:
	cargo update -p cep85
	cargo update -p cep85-test-contract
	cargo update -p tests
