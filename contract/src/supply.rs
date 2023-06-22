use alloc::string::ToString;
use casper_types::U256;

use crate::{
    constants::SUPPLY,
    utils::{get_dictionary_value_from_key, set_dictionary_value_for_key},
};

pub fn write_supply_of(id: &U256, amount: U256) {
    set_dictionary_value_for_key(SUPPLY, &id.to_string(), &amount)
}

pub fn read_supply_of(id: &U256) -> U256 {
    get_dictionary_value_from_key(SUPPLY, &id.to_string()).unwrap_or_default()
}
