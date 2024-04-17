use alloc::string::ToString;
use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{
    bytesrepr::{FromBytes, ToBytes},
    CLTyped, CLValue, U256,
};

use crate::{
    constants::{DICT_SUPPLY, DICT_TOTAL_SUPPLY},
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

pub fn check_token_exists_and_read_total_supply_of<T>(id: U256) -> U256
where
    T: CLTyped + FromBytes + ToBytes,
{
    let total_supply = read_total_supply_of(&id);
    if total_supply == U256::from(0_u32) {
        runtime::ret(CLValue::from_t::<Option<T>>(None).unwrap_or_revert());
    } else {
        total_supply
    }
}
