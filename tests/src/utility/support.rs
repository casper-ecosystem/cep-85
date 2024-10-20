use casper_engine_test_support::{LmdbWasmTestBuilder, DEFAULT_ACCOUNTS};
use casper_event_standard::EVENTS_DICT;
use casper_execution_engine::{engine_state::Error as EngineStateError, execution::ExecError};
use casper_types::{
    account::AccountHash,
    bytesrepr::{Bytes, FromBytes},
    AddressableEntityHash, ApiError, CLTyped, EntityAddr, Key, PublicKey,
};
use std::fmt::Debug;

pub fn assert_expected_error(actual_error: EngineStateError, error_code: u16, reason: &str) {
    let actual = format!("{actual_error:?}");
    let expected = format!(
        "{:?}",
        EngineStateError::Exec(ExecError::Revert(ApiError::User(error_code)))
    );

    assert_eq!(
        actual, expected,
        "Error should match {error_code} with reason: {reason}"
    )
}

pub fn get_dictionary_value_from_key<T: CLTyped + FromBytes>(
    builder: &mut LmdbWasmTestBuilder,
    contract_hash: &AddressableEntityHash,
    dictionary_name: &str,
    dictionary_key: &str,
) -> T {
    let seed_uref = *builder
        .get_entity_with_named_keys_by_entity_hash(*contract_hash)
        .expect("must have contract")
        .named_keys()
        .get(dictionary_name)
        .expect("must have key")
        .as_uref()
        .expect("must convert to seed uref");

    builder
        .query_dictionary_item(None, seed_uref, dictionary_key)
        .expect("should have dictionary value")
        .as_cl_value()
        .expect("T should be CLValue")
        .to_owned()
        .into_t()
        .unwrap()
}

pub fn get_event<T: FromBytes + CLTyped + Debug>(
    builder: &mut LmdbWasmTestBuilder,
    contract_hash: &AddressableEntityHash,
    index: u32,
) -> T {
    let bytes: Bytes =
        get_dictionary_value_from_key(builder, contract_hash, EVENTS_DICT, &index.to_string());
    let (event, bytes) = T::from_bytes(&bytes).unwrap();
    assert!(bytes.is_empty());
    event
}

pub fn get_test_account(ending_string_index: &str) -> (Key, AccountHash, PublicKey) {
    let index = ending_string_index
        .chars()
        .next_back()
        .unwrap()
        .to_digit(10)
        .unwrap_or_default() as usize;

    let accounts = if let Some(account) = DEFAULT_ACCOUNTS.clone().get(index) {
        let public_key = account.public_key().clone();
        let account_hash = public_key.to_account_hash();
        let entity_addr = Key::AddressableEntity(EntityAddr::Account(account_hash.value()));
        Some((entity_addr, account_hash, public_key))
    } else {
        None
    };

    match accounts {
        Some(account) => account,
        None => {
            panic!("No account found for index {}", index);
        }
    }
}
