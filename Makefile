ALL_CONTRACTS = cep85 cep85-test-contract
CONTRACT_TARGET_DIR = contracts/target/wasm32-unknown-unknown/release
PINNED_TOOLCHAIN := $(shell cat contracts/rust-toolchain)

prepare:
	rustup target add wasm32-unknown-unknown
	rustup component add clippy --toolchain ${PINNED_TOOLCHAIN}
	rustup component add rustfmt --toolchain ${PINNED_TOOLCHAIN}

.PHONY:	build-contract
build-contract:
	cd contracts/cep85 && cargo build --release
	wasm-strip $(CONTRACT_TARGET_DIR)/cep85.wasm

.PHONY:	build-all-contracts
build-all-contracts:
	cd contracts && cargo build --release $(patsubst %,-p %, $(ALL_CONTRACTS))
	$(foreach WASM, $(ALL_CONTRACTS), wasm-strip $(CONTRACT_TARGET_DIR)/$(subst -,_,$(WASM)).wasm ;)

setup-test: build-all-contracts
	mkdir -p tests/wasm
	cp $(CONTRACT_TARGET_DIR)/cep85.wasm tests/wasm
	cp $(CONTRACT_TARGET_DIR)/cep85_test_contract.wasm tests/wasm

test: setup-test
	cd tests && cargo test --release

clippy:
	cd contracts && cargo clippy --bins -- -D warnings
	cd contracts && cargo clippy --lib -- -D warnings
	cd contracts && cargo clippy --lib --no-default-features -- -D warnings
	cd tests && cargo clippy --all-targets -- -D warnings

check-lint: clippy
	cd contracts && cargo fmt -- --check
	cd tests && cargo +$(PINNED_TOOLCHAIN) fmt -- --check

format:
	cd contracts && cargo fmt
	cd tests && cargo +$(PINNED_TOOLCHAIN) fmt

clean:
	cd contracts && cargo clean
	cd tests && cargo clean
	rm -rf tests/wasm
