#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::ToString;
use casper_contract::{self, contract_api::storage};
use casper_types::EntryPoints;

const CEP1155_TEST_CONTRACT_HASH_KEY_NAME: &str = "cep1155_test_contract_hash";

#[no_mangle]
pub extern "C" fn call() {
    let entry_points = EntryPoints::new();

    let (_contract_hash, _version) = storage::new_contract(
        entry_points,
        None,
        Some(CEP1155_TEST_CONTRACT_HASH_KEY_NAME.to_string()),
        None,
    );
}
