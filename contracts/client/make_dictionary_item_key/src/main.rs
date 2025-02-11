#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

extern crate alloc;
use alloc::string::String;

use casper_contract::{
    contract_api::{
        runtime::{put_key, revert},
        storage::new_uref,
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{Key, U256};
use cep85::{
    constants::{
        ARG_ID, ARG_OPERATOR, ARG_OWNER, ARG_SESSION_NAMED_KEY_NAME, DEFAULT_DICT_ITEM_KEY_NAME,
    },
    error::Cep85Error,
    utils::{
        get_named_arg_with_user_errors, get_optional_named_arg_with_user_errors,
        make_dictionary_item_key,
    },
};

#[no_mangle]
pub extern "C" fn call() {
    let owner: Key =
        get_named_arg_with_user_errors(ARG_OWNER, Cep85Error::MissingKey, Cep85Error::InvalidKey)
            .unwrap_or_revert();

    let id: Option<U256> =
        get_optional_named_arg_with_user_errors(ARG_ID, Cep85Error::InvalidValue);

    let dictionary_item_key: String = match id {
        Some(id) => make_dictionary_item_key(&owner, &id),
        None => {
            let operator: Option<Key> =
                get_optional_named_arg_with_user_errors(ARG_OPERATOR, Cep85Error::InvalidOperator);
            match operator {
                Some(operator) => make_dictionary_item_key(&owner, &operator),
                None => revert(Cep85Error::InvalidOperator),
            }
        }
    };

    let session_named_key_name: Option<String> = get_optional_named_arg_with_user_errors(
        ARG_SESSION_NAMED_KEY_NAME,
        Cep85Error::InvalidValue,
    );
    let session_named_key_name: &str = session_named_key_name
        .as_deref()
        .unwrap_or(DEFAULT_DICT_ITEM_KEY_NAME);
    put_key(session_named_key_name, new_uref(dictionary_item_key).into());
}
