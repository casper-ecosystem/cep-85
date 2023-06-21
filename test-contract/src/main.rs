#![no_std]
#![no_main]

extern crate alloc;

use alloc::{
    string::{String, ToString},
    vec,
};
use casper_contract::{
    self,
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    bytesrepr::ToBytes, runtime_args, CLType, CLTyped, ContractHash, EntryPoint, EntryPointAccess,
    EntryPointType, EntryPoints, Key, Parameter, RuntimeArgs, U256,
};
use cep1155::constants::{ARG_ACCOUNT, ENTRY_POINT_BALANCE_OF};

const CEP1155_TEST_CONTRACT_HASH_KEY_NAME: &str = "cep1155_test_contract_hash";
const ARG_TOKEN_CONTRACT_RUNTIME: &str = "token_contract";
const RESULT_KEY: &str = "result";
const ENTRY_POINT_CHECK_BALANCE: &str = "check_balance_of";

fn store_result<T: CLTyped + ToBytes>(result: T) {
    match runtime::get_key(RESULT_KEY) {
        Some(Key::URef(uref)) => storage::write(uref, result),
        Some(_) => unreachable!(),
        None => {
            let new_uref = storage::new_uref(result);
            runtime::put_key(RESULT_KEY, new_uref.into());
        }
    }
}

#[no_mangle]
extern "C" fn check_balance_of() {
    let token_contract: ContractHash = ContractHash::new(
        runtime::get_named_arg::<Key>(ARG_TOKEN_CONTRACT_RUNTIME)
            .into_hash()
            .unwrap_or_revert(),
    );
    let account: Key = runtime::get_named_arg(ARG_ACCOUNT);

    let balance_args = runtime_args! {
        ARG_ACCOUNT => account,
    };
    let result: U256 = runtime::call_contract(token_contract, ENTRY_POINT_BALANCE_OF, balance_args);

    store_result(result);
}

// TODO Add rest of entrypoints
#[no_mangle]
pub extern "C" fn call() {
    let mut entry_points = EntryPoints::new();

    let check_balance_of_entrypoint = EntryPoint::new(
        String::from(ENTRY_POINT_CHECK_BALANCE),
        vec![
            Parameter::new(ARG_TOKEN_CONTRACT_RUNTIME, ContractHash::cl_type()),
            Parameter::new(ARG_ACCOUNT, Key::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    entry_points.add_entry_point(check_balance_of_entrypoint);

    let (_contract_hash, _version) = storage::new_contract(
        entry_points,
        None,
        Some(CEP1155_TEST_CONTRACT_HASH_KEY_NAME.to_string()),
        None,
    );
}
