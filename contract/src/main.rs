#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use core::convert::TryInto;

use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};
use casper_contract::{
    contract_api::{
        runtime::{self, get_key, get_named_arg, put_key, revert},
        storage,
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    contracts::NamedKeys,
    runtime_args,
    system::{handle_payment::ARG_ACCOUNT, mint::ARG_ID},
    Key, RuntimeArgs,
};
use cep1155::{
    constants::{
        ENTRY_POINT_INIT, EVENTS_MODE, IDENTIFIER_MODE, NAME, NUMBER_OF_MINTED_TOKENS,
        PACKAGE_HASH, PREFIX_ACCESS_KEY_NAME, PREFIX_CONTRACT_NAME, PREFIX_CONTRACT_PACKAGE_NAME,
        PREFIX_CONTRACT_VERSION, TOTAL_TOKEN_SUPPLY,
    },
    entry_points::generate_entry_points,
    error::Cep1155Error,
    events::init_events,
    modalities::{TokenIdentifier, TokenIdentifierMode},
    utils,
};

/// Initiates the contracts states. Only used by the installer call,
/// later calls will cause it to revert.
#[no_mangle]
pub extern "C" fn init() {
    // TODO Change to admin check
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
pub extern "C" fn balance_of() {
    let account: Key = runtime::get_named_arg(ARG_ACCOUNT);
    let token_id: u64 = runtime::get_named_arg(ARG_ID);
    let identifier_mode: TokenIdentifierMode = utils::get_stored_value_with_user_errors::<u8>(
        IDENTIFIER_MODE,
        Cep1155Error::MissingIdentifierMode,
        Cep1155Error::InvalidIdentifierMode,
    )
    .try_into()
    .unwrap_or_revert();
    let token_id: TokenIdentifier = match identifier_mode {
        TokenIdentifierMode::Ordinal => TokenIdentifier::Index(token_id),
        // TokenIdentifierMode::Hash => TokenIdentifier::Hash(base16::encode_lower(
        //     &runtime::blake2b(token_metadata.clone()),
        // )),
    };
    //  *balances.get(&(account, token_id)).unwrap_or(&0)
}

// pub fn balance_of_batch(balances: &Balances, accounts: Vec<String>, ids: Vec<u32>) -> Vec<u32> {
//     let mut result = Vec::new();
//     for account in accounts {
//         for id in &ids {}
//     }
//     result
// }

// pub fn set_approval_for_all(
//     operator_approval: &mut OperatorApproval,
//     operator: String,
//     approved: bool,
// ) {
//     let caller = runtime::get_caller();
//     operator_approval.insert((caller, operator), approved);
// }

// pub fn is_approved_for_all(
//     operator_approval: &OperatorApproval,
//     account: String,
//     operator: String,
// ) -> bool {
//     *operator_approval
//         .get(&(account, operator))
//         .unwrap_or(&false)
// }

// pub fn safe_transfer_from(
//     balances: &mut Balances,
//     from: String,
//     to: String,
//     token_id: u32,
//     amount: u32,
//     data: String,
// ) {
//     let sender_balance = balances.entry((from.clone(), token_id)).or_insert(0);
//     let recipient_balance = balances.entry((to.clone(), token_id)).or_insert(0);

//     assert!(*sender_balance >= amount, "Insufficient balance");

//     *sender_balance -= amount;
//     *recipient_balance += amount;

//     external_function(data);
// }

// pub fn safe_batch_transfer_from(
//     balances: &mut Balances,
//     from: String,
//     to: String,
//     ids: Vec<u32>,
//     amounts: Vec<u32>,
//     data: String,
// ) {
//     assert_eq!(ids.len(), amounts.len(), "Mismatched vector lengths");

//     for (i, token_id) in ids.iter().enumerate() {
//         let amount = amounts[i];
//         let sender_balance = balances.entry((from.clone(), *token_id)).or_insert(0);
//         let recipient_balance = balances.entry((to.clone(), *token_id)).or_insert(0);

//         assert!(*sender_balance >= amount, "Insufficient balance");

//         *sender_balance -= amount;
//         *recipient_balance += amount;
//     }

//     external_function(data);
// }

// fn external_function(data: String) {
//     // Traitement des données supplémentaires, à implémenter
// }

#[no_mangle]
pub extern "C" fn call() {
    install_contract()
}
