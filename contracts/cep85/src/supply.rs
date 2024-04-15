use alloc::string::ToString;
use casper_contract::contract_api::runtime::revert;
use casper_types::U256;

use crate::{
    constants::{DICT_SUPPLY, DICT_TOTAL_SUPPLY},
    error::Cep85Error,
    utils::{get_dictionary_value_from_key, set_dictionary_value_for_key},
};

pub fn write_supply_of(id: &U256, amount: &U256) {
    set_dictionary_value_for_key(DICT_SUPPLY, &id.to_string(), amount)
}

pub fn read_supply_of(id: &U256) -> U256 {
    get_dictionary_value_from_key(DICT_SUPPLY, &id.to_string()).unwrap_or_default()
}

pub fn write_total_supply_of(id: &U256, amount: &U256) {
    set_dictionary_value_for_key(DICT_TOTAL_SUPPLY, &id.to_string(), amount)
}

pub fn read_total_supply_of(id: &U256) -> U256 {
    get_dictionary_value_from_key(DICT_TOTAL_SUPPLY, &id.to_string()).unwrap_or_default()
}

pub fn check_token_exists_and_read_total_supply_of(id: U256) -> U256 {
    let total_supply = read_total_supply_of(&id);
    if total_supply == U256::from(0_u32) {
        revert(Cep85Error::NonSuppliedTokenId);
    }
    total_supply
}
