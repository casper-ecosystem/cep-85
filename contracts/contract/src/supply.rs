use crate::{
    constants::{DICT_SUPPLY, DICT_TOTAL_SUPPLY},
    utils::{get_dictionary_value_from_key, set_dictionary_value_for_key},
};
use alloc::string::ToString;
use casper_types::U256;

pub fn write_supply_of(id: &U256, amount: &U256) {
    set_dictionary_value_for_key(DICT_SUPPLY, &id.to_string(), amount)
}

pub fn read_supply_of(id: &U256) -> U256 {
    get_dictionary_value_from_key(DICT_SUPPLY, &id.to_string()).unwrap_or_default()
}

pub fn write_total_supply_of(id: &U256, amount: &U256) {
    set_dictionary_value_for_key(DICT_TOTAL_SUPPLY, &id.to_string(), amount)
}

pub fn read_total_supply_of(id: &U256) -> Option<U256> {
    get_dictionary_value_from_key(DICT_TOTAL_SUPPLY, &id.to_string())
}
