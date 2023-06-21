#![no_std]
#![no_main]

extern crate alloc;

use alloc::{boxed::Box, string::ToString, vec, vec::Vec};
use casper_contract::{
    self,
    contract_api::{
        runtime::{self, get_named_arg, put_key},
        storage,
    },
};
use casper_types::{
    runtime_args, system::auction::ARG_AMOUNT, CLType, ContractHash, EntryPoint, EntryPointAccess,
    EntryPointType, EntryPoints, Key, Parameter, RuntimeArgs, U256,
};
use cep1155::constants::{
    ARG_ACCOUNT, ARG_ACCOUNTS, ARG_AMOUNTS, ARG_APPROVED, ARG_FROM, ARG_ID, ARG_IDS, ARG_OPERATOR,
    ARG_TO, ARG_TOKEN_CONTRACT, ENTRY_POINT_BALANCE_OF, ENTRY_POINT_BALANCE_OF_BATCH,
    ENTRY_POINT_INIT, ENTRY_POINT_IS_APPROVED_FOR_ALL, ENTRY_POINT_SAFE_BATCH_TRANSFER_FROM,
    ENTRY_POINT_SAFE_TRANSFER, ENTRY_POINT_SET_APPROVAL_FOR_ALL,
};
use utils::{get_token_contract, store_result};
mod utils;

const CEP1155_TEST_PACKAGE_NAME: &str = "cep1155_test_contract_package_hash";
const RESULT_KEY: &str = "result";

const ENTRY_POINT_CHECK_BALANCE: &str = "check_balance_of";
const ENTRY_POINT_CHECK_BALANCE_OF_BATCH: &str = "check_balance_of_batch";
const ENTRY_POINT_CHECK_SET_APPROVAL_FOR_ALL: &str = "check_set_approval_for_all";
const ENTRY_POINT_CHECK_IS_APPROVED_FOR_ALL: &str = "check_is_approved_for_all";
const ENTRY_POINT_CHECK_SAFE_TRANSFER_FROM: &str = "check_safe_transfer_from";
const ENTRY_POINT_CHECK_SAFE_BATCH_TRANSFER_FROM: &str = "check_safe_batch_transfer_from";

#[no_mangle]
pub extern "C" fn init() {
    let token_contract = get_named_arg::<Key>(ARG_TOKEN_CONTRACT);
    put_key(ARG_TOKEN_CONTRACT, token_contract);
}

#[no_mangle]
extern "C" fn check_balance_of() {
    let token_contract: ContractHash = get_token_contract();
    let account: Key = get_named_arg(ARG_ACCOUNT);
    let id: U256 = get_named_arg(ARG_ID);
    let balance_args = runtime_args! {
        ARG_ACCOUNT => account,
        ARG_ID => id,
    };
    let result: U256 = runtime::call_contract(token_contract, ENTRY_POINT_BALANCE_OF, balance_args);
    store_result(result);
}

#[no_mangle]
extern "C" fn check_balance_of_batch() {
    let token_contract: ContractHash = get_token_contract();
    let accounts: Vec<Key> = get_named_arg(ARG_ACCOUNTS);
    let ids: Vec<U256> = get_named_arg(ARG_IDS);
    let balance_of_batch_args = runtime_args! {
        ARG_ACCOUNTS => accounts,
        ARG_IDS => ids,
    };
    let result: Vec<U256> = runtime::call_contract(
        token_contract,
        ENTRY_POINT_BALANCE_OF_BATCH,
        balance_of_batch_args,
    );
    store_result(result);
}

#[no_mangle]
extern "C" fn check_set_approval_for_all() {
    let token_contract: ContractHash = get_token_contract();
    let operator: Key = get_named_arg(ARG_OPERATOR);
    let approved: bool = get_named_arg(ARG_APPROVED);
    let set_approval_for_all_args = runtime_args! {
        ARG_OPERATOR => operator,
        ARG_APPROVED => approved,
    };
    runtime::call_contract::<()>(
        token_contract,
        ENTRY_POINT_SET_APPROVAL_FOR_ALL,
        set_approval_for_all_args,
    );
}

#[no_mangle]
extern "C" fn check_is_approved_for_all() {
    let token_contract: ContractHash = get_token_contract();
    let account: Key = get_named_arg(ARG_ACCOUNT);
    let operator: Key = get_named_arg(ARG_OPERATOR);
    let is_approved_for_all_args = runtime_args! {
        ARG_ACCOUNT => account,
        ARG_OPERATOR => operator,
    };
    let result: bool = runtime::call_contract(
        token_contract,
        ENTRY_POINT_IS_APPROVED_FOR_ALL,
        is_approved_for_all_args,
    );
    store_result(result);
}

#[no_mangle]
extern "C" fn check_safe_transfer_from() {
    let token_contract: ContractHash = get_token_contract();
    let from: Key = get_named_arg(ARG_FROM);
    let to: Key = get_named_arg(ARG_TO);
    let id: U256 = get_named_arg(ARG_ID);
    let amount: U256 = get_named_arg(ARG_AMOUNT);
    let safe_transfer_from_args = runtime_args! {
        ARG_FROM => from,
        ARG_TO => to,
        ARG_ID => id,
        ARG_AMOUNT => amount,
    };
    runtime::call_contract::<()>(
        token_contract,
        ENTRY_POINT_SAFE_TRANSFER,
        safe_transfer_from_args,
    );
}

#[no_mangle]
extern "C" fn check_safe_batch_transfer_from() {
    let token_contract: ContractHash = get_token_contract();
    let from: Key = get_named_arg(ARG_FROM);
    let to: Key = get_named_arg(ARG_TO);
    let ids: Vec<U256> = get_named_arg(ARG_IDS);
    let amounts: Vec<U256> = get_named_arg(ARG_AMOUNTS);
    let safe_batch_transfer_from_args = runtime_args! {
        ARG_FROM => from,
        ARG_TO => to,
        ARG_IDS => ids,
        ARG_AMOUNTS => amounts,
    };
    runtime::call_contract::<()>(
        token_contract,
        ENTRY_POINT_SAFE_BATCH_TRANSFER_FROM,
        safe_batch_transfer_from_args,
    );
}

#[no_mangle]
pub extern "C" fn call() {
    let mut entry_points = EntryPoints::new();
    let init = EntryPoint::new(
        ENTRY_POINT_INIT,
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );
    let check_balance_of_entrypoint = EntryPoint::new(
        ENTRY_POINT_CHECK_BALANCE,
        vec![
            Parameter::new(ARG_ACCOUNT, CLType::Key),
            Parameter::new(ARG_ID, CLType::U256),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );
    let check_balance_of_batch_entrypoint = EntryPoint::new(
        ENTRY_POINT_CHECK_BALANCE_OF_BATCH,
        vec![
            Parameter::new(ARG_ACCOUNTS, CLType::List(Box::new(CLType::Key))),
            Parameter::new(ARG_IDS, CLType::List(Box::new(CLType::U256))),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );
    let check_set_approval_for_all_entrypoint = EntryPoint::new(
        ENTRY_POINT_CHECK_SET_APPROVAL_FOR_ALL,
        vec![
            Parameter::new(ARG_OPERATOR, CLType::Key),
            Parameter::new(ARG_APPROVED, CLType::Bool),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );
    let check_is_approved_for_all_entrypoint = EntryPoint::new(
        ENTRY_POINT_CHECK_IS_APPROVED_FOR_ALL,
        vec![
            Parameter::new(ARG_ACCOUNT, CLType::Key),
            Parameter::new(ARG_OPERATOR, CLType::Key),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );
    let check_safe_transfer_from_entrypoint = EntryPoint::new(
        ENTRY_POINT_CHECK_SAFE_TRANSFER_FROM,
        vec![
            Parameter::new(ARG_FROM, CLType::Key),
            Parameter::new(ARG_TO, CLType::Key),
            Parameter::new(ARG_ID, CLType::U256),
            Parameter::new(ARG_AMOUNT, CLType::U256),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );
    let check_safe_batch_transfer_from_entrypoint = EntryPoint::new(
        ENTRY_POINT_CHECK_SAFE_BATCH_TRANSFER_FROM,
        vec![
            Parameter::new(ARG_FROM, CLType::Key),
            Parameter::new(ARG_TO, CLType::Key),
            Parameter::new(ARG_IDS, CLType::List(Box::new(CLType::U256))),
            Parameter::new(ARG_AMOUNTS, CLType::List(Box::new(CLType::U256))),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );
    entry_points.add_entry_point(init);
    entry_points.add_entry_point(check_balance_of_entrypoint);
    entry_points.add_entry_point(check_balance_of_batch_entrypoint);
    entry_points.add_entry_point(check_set_approval_for_all_entrypoint);
    entry_points.add_entry_point(check_is_approved_for_all_entrypoint);
    entry_points.add_entry_point(check_safe_transfer_from_entrypoint);
    entry_points.add_entry_point(check_safe_batch_transfer_from_entrypoint);
    let (contract_hash, _version) = storage::new_contract(
        entry_points,
        None,
        Some(CEP1155_TEST_PACKAGE_NAME.to_string()),
        None,
    );
    let token_contract = get_named_arg::<Key>(ARG_TOKEN_CONTRACT);
    // Call contract to initialize it
    let init_args = runtime_args! {
        ARG_TOKEN_CONTRACT => token_contract,
    };
    runtime::call_contract::<()>(contract_hash, ENTRY_POINT_INIT, init_args);
}
