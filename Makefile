ALL_CONTRACTS = cep-1155 cep-1155-test-contract
CONTRACT_TARGET_DIR = target/wasm32-unknown-unknown/release
PINNED_TOOLCHAIN := $(shell cat rust-toolchain)

prepare:
	rustup target add wasm32-unknown-unknown
	rustup component add clippy --toolchain ${PINNED_TOOLCHAIN}
	rustup component add rustfmt --toolchain ${PINNED_TOOLCHAIN}

	.PHONY:	build-contract
build-contract:
	cargo build --release --target wasm32-unknown-unknown $(patsubst %,-p %, $(ALL_CONTRACTS))
	$(foreach WASM, $(ALL_CONTRACTS), wasm-strip $(CONTRACT_TARGET_DIR)/$(subst -,_,$(WASM)).wasm ;)

test: build-contract
	mkdir -p tests/wasm
	cp $(CONTRACT_TARGET_DIR)/cep_1155.wasm tests/wasm
		cp $(CONTRACT_TARGET_DIR)/cep_1155_test_contract.wasm tests/wasm
	cd tests && cargo test

clippy:
	cd contract && cargo clippy --target wasm32-unknown-unknown --bins -- -D warnings
	cd contract && cargo clippy --lib -- -D warnings
	cd contract && cargo clippy --no-default-features --lib -- -D warnings
	cd test-contract && cargo clippy --target wasm32-unknown-unknown -- -D warnings
	cd tests && cargo clippy --all-targets -- -D warnings

check-lint: clippy
	cd contract && cargo fmt -- --check
	cd test-contract && cargo fmt -- --check
	cd tests && cargo fmt -- --check

lint: clippy
	cd contract && cargo fmt
	cd test-contract && cargo fmt
	cd tests && cargo fmt

clean:
	cd contract && cargo clean
	cd test-contract && cargo clean
	cd tests && cargo clean
	rm -rf tests/wasm
