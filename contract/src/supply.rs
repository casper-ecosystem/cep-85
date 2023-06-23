use alloc::string::ToString;
use casper_types::U256;

use crate::{
    constants::{SUPPLY, TOTAL_SUPPLY},
    utils::{get_dictionary_value_from_key, set_dictionary_value_for_key},
};

pub fn write_supply_of(id: &U256, amount: &U256) {
    set_dictionary_value_for_key(SUPPLY, &id.to_string(), amount)
}

pub fn read_supply_of(id: &U256) -> U256 {
    get_dictionary_value_from_key(SUPPLY, &id.to_string()).unwrap_or_default()
}

pub fn write_total_supply_of(id: &U256, amount: &U256) {
    set_dictionary_value_for_key(TOTAL_SUPPLY, &id.to_string(), amount)
}

pub fn read_total_supply_of(id: &U256) -> U256 {
    // We set max supply to 1 if the token is not already in the dict TOTAL_SUPPLY
    get_dictionary_value_from_key(TOTAL_SUPPLY, &id.to_string()).unwrap_or(U256::from(1_u32))
}
