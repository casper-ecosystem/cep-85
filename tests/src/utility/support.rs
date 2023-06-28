use casper_engine_test_support::WasmTestBuilder;
use casper_event_standard::EVENTS_DICT;
use casper_execution_engine::{
    core::{engine_state::Error as EngineStateError, execution},
    storage::global_state::in_memory::InMemoryGlobalState,
};
use casper_types::{
    bytesrepr::{Bytes, FromBytes},
    ApiError, CLTyped, Key,
};
use std::fmt::Debug;

pub fn assert_expected_error(actual_error: EngineStateError, error_code: u16, reason: &str) {
    let actual = format!("{actual_error:?}");
    let expected = format!(
        "{:?}",
        EngineStateError::Exec(execution::Error::Revert(ApiError::User(error_code)))
    );

    assert_eq!(
        actual, expected,
        "Error should match {error_code} with reason: {reason}"
    )
}

pub fn get_dictionary_value_from_key<T: CLTyped + FromBytes>(
    builder: &WasmTestBuilder<InMemoryGlobalState>,
    contract_key: &Key,
    dictionary_name: &str,
    dictionary_key: &str,
) -> T {
    let seed_uref = *builder
        .query(None, *contract_key, &[])
        .expect("must have contract")
        .as_contract()
        .expect("must convert contract")
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
    builder: &WasmTestBuilder<InMemoryGlobalState>,
    contract_key: &Key,
    index: u32,
) -> T {
    let bytes: Bytes =
        get_dictionary_value_from_key(builder, contract_key, EVENTS_DICT, &index.to_string());
    let (event, bytes) = T::from_bytes(&bytes).unwrap();
    assert!(bytes.is_empty());
    event
}
