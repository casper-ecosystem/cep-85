//! Implementation of uri.
use alloc::string::{String, ToString};
use casper_contract::contract_api::runtime::revert;
use casper_types::U256;
use core::ops::Deref;

use crate::{
    constants::{ARG_URI, DICT_TOKEN_URI},
    error::Cep85Error,
    utils::{
        get_dictionary_value_from_key, get_stored_value_with_user_errors, replace_token_id_in_uri,
        set_dictionary_value_for_key,
    },
};

pub fn write_uri_of(id: &U256, raw_uri: &str) {
    let uri = replace_token_id_in_uri(raw_uri, id);
    set_dictionary_value_for_key(DICT_TOKEN_URI, &id.to_string(), &uri.deref());
}

pub fn read_uri_of(id: Option<U256>) -> String {
    let uri: String = match id {
        Some(id) => get_dictionary_value_from_key(DICT_TOKEN_URI, &id.to_string())
            .filter(|value: &String| !value.is_empty())
            .unwrap_or_else(|| {
                let global_uri: String = get_stored_value_with_user_errors(
                    ARG_URI,
                    Cep85Error::MissingUri,
                    Cep85Error::InvalidUri,
                );
                replace_token_id_in_uri(&global_uri, &id)
            }),
        None => get_stored_value_with_user_errors(
            ARG_URI,
            Cep85Error::MissingUri,
            Cep85Error::InvalidUri,
        ),
    };
    if uri.is_empty() {
        revert(Cep85Error::MissingUri);
    }
    uri
}
