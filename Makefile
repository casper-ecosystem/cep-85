ALL_CONTRACTS = cep85 cep85-test-contract
CONTRACT_TARGET_DIR = contracts/target/wasm32-unknown-unknown/release
PINNED_TOOLCHAIN := $(shell cat contracts/rust-toolchain)
RUSTFLAGS := -C target-cpu=mvp
CARGO_BUILD_FLAGS := -Z build-std=std,panic_abort
WASM_OUTPUT_DIR := tests/wasm

prepare:
	rustup install $(PINNED_TOOLCHAIN)
	rustup target add wasm32-unknown-unknown --toolchain ${PINNED_TOOLCHAIN}
	rustup component add clippy --toolchain ${PINNED_TOOLCHAIN}
	rustup component add rustfmt --toolchain ${PINNED_TOOLCHAIN}
	rustup component add rust-src --toolchain $(PINNED_TOOLCHAIN)

.PHONY:	build-contract
build-contract:
	cd contracts/cep85 && RUSTFLAGS="$(RUSTFLAGS)" cargo +$(PINNED_TOOLCHAIN) build --release --target wasm32-unknown-unknown $(CARGO_BUILD_FLAGS)
	wasm-strip $(CONTRACT_TARGET_DIR)/cep85.wasm

.PHONY:	build-all-contracts
build-all-contracts:
	cd contracts && RUSTFLAGS="$(RUSTFLAGS)" cargo +$(PINNED_TOOLCHAIN) build --release --target wasm32-unknown-unknown $(CARGO_BUILD_FLAGS)
	$(foreach WASM, $(ALL_CONTRACTS), wasm-strip $(CONTRACT_TARGET_DIR)/$(subst -,_,$(WASM)).wasm ;)

setup-test: build-all-contracts
	mkdir -p tests/wasm
	cp $(CONTRACT_TARGET_DIR)/cep85.wasm tests/wasm
	cp $(CONTRACT_TARGET_DIR)/cep85_test_contract.wasm tests/wasm

test: setup-test
	cd tests && cargo test

clippy:
	cd contracts && cargo clippy --bins --target wasm32-unknown-unknown -- -D warnings
	cd contracts && cargo clippy --lib --target wasm32-unknown-unknown -- -D warnings
	cd contracts && cargo clippy --lib --target wasm32-unknown-unknown --no-default-features -- -D warnings
	cd tests && cargo +stable clippy --all-targets -- -D warnings

check-lint: clippy
	cd contracts && cargo fmt -- --check
	cd tests && cargo +stable fmt -p tests -- --check

format:
	cd contracts && cargo fmt
	cd tests && cargo +stable fmt -p tests

clean:
	cd contracts && cargo clean
	cd tests && cargo clean
	rm -rf tests/wasm
	rm -rf $(WASM_OUTPUT_DIR)
	rm -rf ./Cargo.lock
