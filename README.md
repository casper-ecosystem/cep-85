# Casper Semi-Fungible Tokens (CEP-1155 Standard)

This repository contains a reference contract implementation and tests for Semi-Fungible Tokens on a Casper network, following the [CEP-1155 standard](https://github.com/casper-network/ceps/pull/1155).

## Preparation

Install the `wasm32-unknown-unknown` Rust target with the following command.

```
make prepare
```

## Building and Testing the Contract

To build the reference fungible token contract and supporting tests, run this command:

```
make test
```

## Locating the Contract Wasm

Find the Wasm for the contract in the following directory:

```
./target/wasm32-unknown-unknown/release/cep1155_token.wasm
```

## A JavaScript Client SDK

## Tutorials

For more information, visit the links below:
