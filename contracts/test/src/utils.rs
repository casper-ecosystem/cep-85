use casper_contract::{
    contract_api::{
        runtime::{self, get_key},
        storage,
    },
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{bytesrepr::ToBytes, AddressableEntityHash, CLTyped, Key};

use crate::{constants::RESULT_KEY, ARG_TOKEN_CONTRACT};

pub fn store_result<T: CLTyped + ToBytes>(result: T) {
    match runtime::get_key(RESULT_KEY) {
        Some(Key::URef(uref)) => storage::write(uref, result),
        Some(_) => unreachable!(),
        None => {
            let new_uref = storage::new_uref(result);
            runtime::put_key(RESULT_KEY, new_uref.into());
        }
    }
}

pub fn get_token_contract() -> AddressableEntityHash {
    let key = get_key(ARG_TOKEN_CONTRACT).unwrap_or_revert();
    key.into_entity_hash().unwrap_or_revert()
}
