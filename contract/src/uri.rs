//! Implementation of allowances.
use alloc::{
    format,
    string::{String, ToString},
};
use casper_types::U256;

use crate::{
    constants::TOKEN_URI,
    utils::{get_dictionary_value_from_key, set_dictionary_value_for_key},
};

pub fn write_uri_of(id: U256, uri: &str) {
    set_dictionary_value_for_key(TOKEN_URI, &id.to_string(), &uri)
}

pub fn read_uri_of(id: U256) -> String {
    // TODO Check if token id exists

    let raw_uri: String =
        get_dictionary_value_from_key(TOKEN_URI, &id.to_string()).unwrap_or_default();
    raw_uri.replace("{id}", &format!("{id}"))
}
