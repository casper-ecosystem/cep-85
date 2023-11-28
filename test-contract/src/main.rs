#![no_std]
#![no_main]

extern crate alloc;

mod constants;
mod utils;

use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use casper_contract::{
    self,
    contract_api::{
        runtime::{call_contract, get_key, get_named_arg, put_key, ret},
        storage,
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    bytesrepr::Bytes, runtime_args, system::auction::ARG_AMOUNT, ApiError, CLType, CLTyped,
    CLValue, ContractHash, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Key,
    Parameter, RuntimeArgs, U256,
};
use cep85::{
    constants::{
        ARG_ACCOUNT, ARG_ACCOUNTS, ARG_AMOUNTS, ARG_DATA, ARG_FROM, ARG_ID, ARG_IDS, ARG_OPERATOR,
        ARG_OWNER, ARG_TO, ARG_TOKEN_CONTRACT, ENTRY_POINT_BALANCE_OF,
        ENTRY_POINT_BALANCE_OF_BATCH, ENTRY_POINT_BATCH_BURN, ENTRY_POINT_BURN, ENTRY_POINT_INIT,
        ENTRY_POINT_IS_APPROVED_FOR_ALL, ENTRY_POINT_IS_NON_FUNGIBLE,
        ENTRY_POINT_SAFE_BATCH_TRANSFER_FROM, ENTRY_POINT_SAFE_TRANSFER_FROM,
        ENTRY_POINT_SUPPLY_OF, ENTRY_POINT_SUPPLY_OF_BATCH, ENTRY_POINT_TOTAL_FUNGIBLE_SUPPLY,
        ENTRY_POINT_TOTAL_SUPPLY_OF, ENTRY_POINT_TOTAL_SUPPLY_OF_BATCH, ENTRY_POINT_URI,
    },
    modalities::TransferFilterContractResult,
};
use constants::{
    ARG_FILTER_CONTRACT_RETURN_VALUE, CEP85_TEST_CONTRACT_NAME, CEP85_TEST_PACKAGE_NAME,
    ENTRY_POINT_CHECK_BALANCE_OF, ENTRY_POINT_CHECK_BALANCE_OF_BATCH,
    ENTRY_POINT_CHECK_IS_APPROVED_FOR_ALL, ENTRY_POINT_CHECK_IS_NON_FUNGIBLE,
    ENTRY_POINT_CHECK_SAFE_BATCH_TRANSFER_FROM, ENTRY_POINT_CHECK_SAFE_TRANSFER_FROM,
    ENTRY_POINT_CHECK_SUPPLY_OF, ENTRY_POINT_CHECK_SUPPLY_OF_BATCH,
    ENTRY_POINT_CHECK_TOTAL_FUNGIBLE_SUPPLY, ENTRY_POINT_CHECK_TOTAL_SUPPLY_OF,
    ENTRY_POINT_CHECK_TOTAL_SUPPLY_OF_BATCH, ENTRY_POINT_CHECK_URI,
    ENTRY_POINT_SET_FILTER_CONTRACT_RETURN_VALUE, ENTRY_POINT_TRANSFER_FILTER_METHOD,
};
use utils::{get_token_contract, store_result};

#[no_mangle]
pub extern "C" fn init() {
    let token_contract = get_named_arg::<Key>(ARG_TOKEN_CONTRACT);
    put_key(ARG_TOKEN_CONTRACT, token_contract);
}

// Update stored value for as a contract filter result value
#[no_mangle]
pub extern "C" fn set_filter_contract_return_value() {
    let value: TransferFilterContractResult = get_named_arg(ARG_FILTER_CONTRACT_RETURN_VALUE);
    let uref = storage::new_uref(value);
    put_key(ARG_FILTER_CONTRACT_RETURN_VALUE, uref.into());
}

// Check that some values are sent by token contract and return a TransferFilterContractResult
#[no_mangle]
pub extern "C" fn can_transfer() {
    let _operator: Key = get_named_arg(ARG_OPERATOR);
    let _from: Key = get_named_arg(ARG_FROM);
    let _to: Key = get_named_arg(ARG_FROM);
    let _ids: Vec<U256> = get_named_arg(ARG_IDS);
    let _amounts: Vec<U256> = get_named_arg(ARG_AMOUNTS);
    let _data: Bytes = get_named_arg(ARG_DATA);

    let key = get_key(ARG_FILTER_CONTRACT_RETURN_VALUE);
    if key.is_none() {
        ret(CLValue::from_t(TransferFilterContractResult::DenyTransfer).unwrap_or_revert());
    }
    let uref = get_key(ARG_FILTER_CONTRACT_RETURN_VALUE)
        .unwrap_or_revert()
        .into_uref();
    let value: TransferFilterContractResult =
        storage::read(uref.unwrap_or_revert_with(ApiError::ValueNotFound))
            .unwrap_or_revert()
            .unwrap_or_default();
    ret(CLValue::from_t(value).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn burn() {
    let token_contract: ContractHash = get_token_contract();
    let owner: Key = get_named_arg(ARG_OWNER);
    let id: U256 = get_named_arg(ARG_ID);
    let amount: U256 = get_named_arg(ARG_AMOUNT);
    let burn_args = runtime_args! {
        ARG_OWNER => owner,
        ARG_ID => id,
        ARG_AMOUNT => amount,
    };
    call_contract::<()>(token_contract, ENTRY_POINT_BURN, burn_args);
}

#[no_mangle]
pub extern "C" fn batch_burn() {
    let token_contract: ContractHash = get_token_contract();
    let owner: Key = get_named_arg(ARG_OWNER);
    let ids: Vec<U256> = get_named_arg(ARG_IDS);
    let amounts: Vec<U256> = get_named_arg(ARG_AMOUNTS);
    let batch_burn_args = runtime_args! {
        ARG_OWNER => owner,
        ARG_IDS => ids,
        ARG_AMOUNTS => amounts,
    };
    call_contract::<()>(token_contract, ENTRY_POINT_BATCH_BURN, batch_burn_args);
}

#[no_mangle]
pub extern "C" fn check_balance_of() {
    let token_contract: ContractHash = get_token_contract();
    let account: Key = get_named_arg(ARG_ACCOUNT);
    let id: U256 = get_named_arg(ARG_ID);
    let balance_args = runtime_args! {
        ARG_ACCOUNT => account,
        ARG_ID => id,
    };
    let result: U256 = call_contract(token_contract, ENTRY_POINT_BALANCE_OF, balance_args);
    store_result(result);
}

#[no_mangle]
pub extern "C" fn check_balance_of_batch() {
    let token_contract: ContractHash = get_token_contract();
    let accounts: Vec<Key> = get_named_arg(ARG_ACCOUNTS);
    let ids: Vec<U256> = get_named_arg(ARG_IDS);
    let balance_of_batch_args = runtime_args! {
        ARG_ACCOUNTS => accounts,
        ARG_IDS => ids,
    };
    let result: Vec<U256> = call_contract(
        token_contract,
        ENTRY_POINT_BALANCE_OF_BATCH,
        balance_of_batch_args,
    );
    store_result(result);
}

#[no_mangle]
pub extern "C" fn check_is_approved_for_all() {
    let token_contract: ContractHash = get_token_contract();
    let account: Key = get_named_arg(ARG_ACCOUNT);
    let operator: Key = get_named_arg(ARG_OPERATOR);
    let is_approved_for_all_args = runtime_args! {
        ARG_ACCOUNT => account,
        ARG_OPERATOR => operator,
    };
    let result: bool = call_contract(
        token_contract,
        ENTRY_POINT_IS_APPROVED_FOR_ALL,
        is_approved_for_all_args,
    );
    store_result(result);
}

#[no_mangle]
pub extern "C" fn check_safe_transfer_from() {
    let token_contract: ContractHash = get_token_contract();
    let from: Key = get_named_arg(ARG_FROM);
    let to: Key = get_named_arg(ARG_TO);
    let id: U256 = get_named_arg(ARG_ID);
    let amount: U256 = get_named_arg(ARG_AMOUNT);
    let data: Bytes = get_named_arg(ARG_DATA);
    let safe_transfer_from_args = runtime_args! {
        ARG_FROM => from,
        ARG_TO => to,
        ARG_ID => id,
        ARG_AMOUNT => amount,
        ARG_DATA => data,
    };
    call_contract::<()>(
        token_contract,
        ENTRY_POINT_SAFE_TRANSFER_FROM,
        safe_transfer_from_args,
    );
}

#[no_mangle]
pub extern "C" fn check_safe_batch_transfer_from() {
    let token_contract: ContractHash = get_token_contract();
    let from: Key = get_named_arg(ARG_FROM);
    let to: Key = get_named_arg(ARG_TO);
    let ids: Vec<U256> = get_named_arg(ARG_IDS);
    let amounts: Vec<U256> = get_named_arg(ARG_AMOUNTS);
    let data: Bytes = get_named_arg(ARG_DATA);
    let safe_batch_transfer_from_args = runtime_args! {
        ARG_FROM => from,
        ARG_TO => to,
        ARG_IDS => ids,
        ARG_AMOUNTS => amounts,
        ARG_DATA => data,
    };
    call_contract::<()>(
        token_contract,
        ENTRY_POINT_SAFE_BATCH_TRANSFER_FROM,
        safe_batch_transfer_from_args,
    );
}

#[no_mangle]
pub extern "C" fn check_supply_of() {
    let token_contract: ContractHash = get_token_contract();
    let id: U256 = get_named_arg(ARG_ID);
    let check_supply_of_args = runtime_args! {
        ARG_ID => id,
    };
    let result: U256 = call_contract(token_contract, ENTRY_POINT_SUPPLY_OF, check_supply_of_args);
    store_result(result);
}

#[no_mangle]
pub extern "C" fn check_supply_of_batch() {
    let token_contract: ContractHash = get_token_contract();
    let ids: Vec<U256> = get_named_arg(ARG_IDS);
    let check_supply_of_batch_args = runtime_args! {
        ARG_IDS => ids,
    };
    let result = call_contract::<Vec<U256>>(
        token_contract,
        ENTRY_POINT_SUPPLY_OF_BATCH,
        check_supply_of_batch_args,
    );
    store_result(result);
}

#[no_mangle]
pub extern "C" fn check_total_supply_of() {
    let token_contract: ContractHash = get_token_contract();
    let id: U256 = get_named_arg(ARG_ID);
    let check_total_supply_of_args = runtime_args! {
        ARG_ID => id,
    };
    let result: U256 = call_contract(
        token_contract,
        ENTRY_POINT_TOTAL_SUPPLY_OF,
        check_total_supply_of_args,
    );
    store_result(result);
}

#[no_mangle]
pub extern "C" fn check_total_supply_of_batch() {
    let token_contract: ContractHash = get_token_contract();
    let ids: Vec<U256> = get_named_arg(ARG_IDS);
    let check_total_supply_of_batch_args = runtime_args! {
        ARG_IDS => ids,
    };
    let result = call_contract::<Vec<U256>>(
        token_contract,
        ENTRY_POINT_TOTAL_SUPPLY_OF_BATCH,
        check_total_supply_of_batch_args,
    );
    store_result(result);
}

#[no_mangle]
pub extern "C" fn check_uri() {
    let token_contract: ContractHash = get_token_contract();
    let id: Option<U256> = get_named_arg(ARG_ID);
    let check_uri_args = if let Some(id) = id {
        runtime_args! {
            ARG_ID => id,
        }
    } else {
        runtime_args! {}
    };
    let result: String = call_contract(token_contract, ENTRY_POINT_URI, check_uri_args);
    store_result(result);
}

#[no_mangle]
pub extern "C" fn check_is_non_fungible() {
    let token_contract: ContractHash = get_token_contract();
    let id: U256 = get_named_arg(ARG_ID);

    let is_non_fungible_args = runtime_args! {
        ARG_ID => id,
    };

    let is_non_fungible_result: bool = call_contract(
        token_contract,
        ENTRY_POINT_IS_NON_FUNGIBLE,
        is_non_fungible_args,
    );
    store_result(is_non_fungible_result);
}

#[no_mangle]
pub extern "C" fn check_total_fungible_supply() {
    let token_contract: ContractHash = get_token_contract();
    let id: U256 = get_named_arg(ARG_ID);

    let total_fungible_supply_args = runtime_args! {
        ARG_ID => id,
    };

    let total_fungible_supply_result: U256 = call_contract(
        token_contract,
        ENTRY_POINT_TOTAL_FUNGIBLE_SUPPLY,
        total_fungible_supply_args,
    );
    store_result(total_fungible_supply_result);
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
    let set_filter_contract_return_value = EntryPoint::new(
        ENTRY_POINT_SET_FILTER_CONTRACT_RETURN_VALUE,
        vec![Parameter::new(
            ARG_FILTER_CONTRACT_RETURN_VALUE,
            TransferFilterContractResult::cl_type(),
        )],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );
    let can_transfer = EntryPoint::new(
        ENTRY_POINT_TRANSFER_FILTER_METHOD,
        vec![
            Parameter::new(ARG_OPERATOR, CLType::Key),
            Parameter::new(ARG_FROM, CLType::Key),
            Parameter::new(ARG_TO, CLType::Key),
            Parameter::new(ARG_IDS, CLType::List(Box::new(CLType::U256))),
            Parameter::new(ARG_AMOUNTS, CLType::List(Box::new(CLType::U256))),
            Parameter::new(ARG_DATA, CLType::List(Box::new(Bytes::cl_type()))),
        ],
        TransferFilterContractResult::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );
    let burn = EntryPoint::new(
        ENTRY_POINT_BURN,
        vec![
            Parameter::new(ARG_OWNER, CLType::Key),
            Parameter::new(ARG_ID, CLType::U256),
            Parameter::new(ARG_AMOUNT, CLType::U256),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    let batch_burn = EntryPoint::new(
        ENTRY_POINT_BATCH_BURN,
        vec![
            Parameter::new(ARG_OWNER, CLType::Key),
            Parameter::new(ARG_IDS, CLType::List(Box::new(CLType::U256))),
            Parameter::new(ARG_AMOUNTS, CLType::List(Box::new(CLType::U256))),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    let check_balance_of = EntryPoint::new(
        ENTRY_POINT_CHECK_BALANCE_OF,
        vec![
            Parameter::new(ARG_ACCOUNT, CLType::Key),
            Parameter::new(ARG_ID, CLType::U256),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );
    let check_balance_of_batch = EntryPoint::new(
        ENTRY_POINT_CHECK_BALANCE_OF_BATCH,
        vec![
            Parameter::new(ARG_ACCOUNTS, CLType::List(Box::new(CLType::Key))),
            Parameter::new(ARG_IDS, CLType::List(Box::new(CLType::U256))),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );
    let check_is_approved_for_all = EntryPoint::new(
        ENTRY_POINT_CHECK_IS_APPROVED_FOR_ALL,
        vec![
            Parameter::new(ARG_ACCOUNT, CLType::Key),
            Parameter::new(ARG_OPERATOR, CLType::Key),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );
    let check_safe_transfer_from = EntryPoint::new(
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
    let check_safe_batch_transfer_from = EntryPoint::new(
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
    let check_supply_of = EntryPoint::new(
        ENTRY_POINT_CHECK_SUPPLY_OF,
        vec![Parameter::new(ARG_ID, CLType::U256)],
        CLType::U256,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );
    let check_total_supply_of = EntryPoint::new(
        ENTRY_POINT_CHECK_TOTAL_SUPPLY_OF,
        vec![Parameter::new(ARG_ID, CLType::U256)],
        CLType::U256,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );
    let check_supply_of_batch = EntryPoint::new(
        ENTRY_POINT_CHECK_SUPPLY_OF_BATCH,
        vec![Parameter::new(
            ARG_IDS,
            CLType::List(Box::new(CLType::U256)),
        )],
        CLType::List(Box::new(CLType::U256)),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );
    let check_total_supply_of_batch = EntryPoint::new(
        ENTRY_POINT_CHECK_TOTAL_SUPPLY_OF_BATCH,
        vec![Parameter::new(
            ARG_IDS,
            CLType::List(Box::new(CLType::U256)),
        )],
        CLType::List(Box::new(CLType::U256)),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );
    let check_uri = EntryPoint::new(
        ENTRY_POINT_CHECK_URI,
        vec![Parameter::new(
            ARG_ID,
            CLType::Option(Box::new(CLType::U256)),
        )],
        CLType::String,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );
    let check_is_non_fungible = EntryPoint::new(
        ENTRY_POINT_CHECK_IS_NON_FUNGIBLE,
        vec![Parameter::new(ARG_ID, CLType::U256)],
        CLType::Bool,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );
    let check_total_fungible_supply = EntryPoint::new(
        ENTRY_POINT_CHECK_TOTAL_FUNGIBLE_SUPPLY,
        vec![Parameter::new(ARG_ID, CLType::U256)],
        CLType::U256,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    entry_points.add_entry_point(init);
    entry_points.add_entry_point(set_filter_contract_return_value);
    entry_points.add_entry_point(can_transfer);
    entry_points.add_entry_point(burn);
    entry_points.add_entry_point(batch_burn);
    entry_points.add_entry_point(check_balance_of);
    entry_points.add_entry_point(check_balance_of_batch);
    entry_points.add_entry_point(check_is_approved_for_all);
    entry_points.add_entry_point(check_safe_transfer_from);
    entry_points.add_entry_point(check_safe_batch_transfer_from);
    entry_points.add_entry_point(check_supply_of);
    entry_points.add_entry_point(check_supply_of_batch);
    entry_points.add_entry_point(check_total_supply_of);
    entry_points.add_entry_point(check_total_supply_of_batch);
    entry_points.add_entry_point(check_uri);
    entry_points.add_entry_point(check_is_non_fungible);
    entry_points.add_entry_point(check_total_fungible_supply);

    let (contract_hash, _version) = storage::new_contract(
        entry_points,
        None,
        Some(CEP85_TEST_PACKAGE_NAME.to_string()),
        None,
    );

    put_key(CEP85_TEST_CONTRACT_NAME, Key::from(contract_hash));

    let token_contract = get_named_arg::<Key>(ARG_TOKEN_CONTRACT);
    // Call contract to initialize it
    let init_args = runtime_args! {
        ARG_TOKEN_CONTRACT => token_contract,
    };
    call_contract::<()>(contract_hash, ENTRY_POINT_INIT, init_args);
}
