//! Implementation of allowances.

use casper_types::Key;

use crate::{
    constants::DICT_OPERATORS,
    utils::{
        get_dictionary_value_from_key, make_dictionary_item_key, set_dictionary_value_for_key,
    },
};

pub fn write_operator(&owner: &Key, &operator: &Key, approved: bool) {
    set_dictionary_value_for_key(
        DICT_OPERATORS,
        &make_dictionary_item_key(&owner, &operator),
        &approved,
    )
}

pub fn read_operator(&owner: &Key, &spender: &Key) -> bool {
    get_dictionary_value_from_key(DICT_OPERATORS, &make_dictionary_item_key(&owner, &spender))
        .unwrap_or_default()
}
