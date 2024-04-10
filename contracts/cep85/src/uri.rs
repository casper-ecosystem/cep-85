//! Implementation of allowances.
use core::ops::Deref;

use alloc::string::{String, ToString};
use casper_contract::contract_api::runtime;
use casper_types::U256;

use crate::{
    constants::DICT_TOKEN_URI,
    utils::{get_dictionary_value_from_key, replace_token_id_in_uri, set_dictionary_value_for_key},
};

pub fn write_uri_of(id: &U256, raw_uri: &str) -> String {
    runtime::print(&raw_uri);
    let uri = replace_token_id_in_uri(raw_uri, id);
    runtime::print(&uri);
    set_dictionary_value_for_key(DICT_TOKEN_URI, &id.to_string(), &uri.deref());
    uri
}

pub fn read_uri_of(id: &U256) -> String {
    get_dictionary_value_from_key(DICT_TOKEN_URI, &id.to_string()).unwrap_or_default()
}
