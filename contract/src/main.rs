#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use alloc::{
    format,
    string::{String, ToString},
};
use casper_contract::{
    contract_api::{
        runtime::{self, get_key, get_named_arg, put_key, revert},
        storage,
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{contracts::NamedKeys, runtime_args, Key, RuntimeArgs};
use cep_1155::{
    constants::{
        ENTRY_POINT_INIT, EVENTS_MODE, NAME, PACKAGE_HASH, PREFIX_ACCESS_KEY_NAME,
        PREFIX_CONTRACT_NAME, PREFIX_CONTRACT_PACKAGE_NAME, PREFIX_CONTRACT_VERSION,
    },
    entry_points::generate_entry_points,
    error::Cep1155Error,
    events::init_events,
    utils,
};

/// Initiates the contracts states. Only used by the installer call,
/// later calls will cause it to revert.
#[no_mangle]
pub extern "C" fn init() {
    if get_key(PACKAGE_HASH).is_some() {
        revert(Cep1155Error::ContractAlreadyInitialized);
    }
    let package_hash = get_named_arg::<Key>(PACKAGE_HASH);
    put_key(PACKAGE_HASH, package_hash);
    init_events();
}

pub fn install_contract() {
    let name: String = get_named_arg(NAME);

    let events_mode: u8 = utils::get_optional_named_arg_with_user_errors(
        EVENTS_MODE,
        Cep1155Error::InvalidEventsMode,
    )
    .unwrap_or(0u8);

    let mut named_keys = NamedKeys::new();
    named_keys.insert(NAME.to_string(), storage::new_uref(name.clone()).into());
    named_keys.insert(
        EVENTS_MODE.to_string(),
        storage::new_uref(events_mode).into(),
    );

    let entry_points = generate_entry_points();

    let hash_key_name = format!("{PREFIX_CONTRACT_PACKAGE_NAME}_{name}");

    let (contract_hash, contract_version) = storage::new_contract(
        entry_points,
        Some(named_keys),
        Some(hash_key_name.clone()),
        Some(format!("{PREFIX_ACCESS_KEY_NAME}_{name}")),
    );

    runtime::put_key(
        &format!("{PREFIX_CONTRACT_NAME}_{name}"),
        contract_hash.into(),
    );
    runtime::put_key(
        &format!("{PREFIX_CONTRACT_VERSION}_{name}"),
        storage::new_uref(contract_version).into(),
    );

    let package_hash = runtime::get_key(&hash_key_name).unwrap_or_revert();

    // Call contract to initialize it
    let init_args = runtime_args! {
        PACKAGE_HASH => package_hash
    };

    runtime::call_contract::<()>(contract_hash, ENTRY_POINT_INIT, init_args);
}

#[no_mangle]
pub extern "C" fn call() {
    install_contract()
}
