use crate::utility::{
    constants::{CEP1155_CONTRACT_WASM, CEP1155_TEST_TOKEN_CONTRACT_NAME, TOKEN_NAME},
    installer_request_builders::{setup, TestContext},
    support::assert_expected_error,
};
use casper_engine_test_support::{
    ExecuteRequestBuilder, InMemoryWasmTestBuilder, DEFAULT_ACCOUNT_ADDR,
    PRODUCTION_RUN_GENESIS_REQUEST,
};

use casper_types::{runtime_args, ContractHash, RuntimeArgs};
use cep1155::{
    constants::{
        BALANCES, ENABLE_MINT_BURN, ENTRY_POINT_INIT, EVENTS_MODE, NAME, OPERATORS, PACKAGE_HASH,
        TRANSFER_FILTER_CONTRACT,
    },
    error::Cep1155Error,
};

#[test]
fn should_install_contract() {
    let (builder, TestContext { cep1155_token, .. }) = setup();
    let contract = builder
        .get_contract(cep1155_token)
        .expect("should have contract");
    let named_keys = contract.named_keys();
    assert!(named_keys.contains_key(PACKAGE_HASH), "{:?}", named_keys);
}

// TODO
#[test]
fn should_have_queryable_properties() {
    let (mut builder, TestContext { cep1155_token, .. }) = setup();
    let name: String = builder.get_value(cep1155_token, NAME);
    let events_mode: u8 = builder.get_value(cep1155_token, EVENTS_MODE);
    let enable_mint_burn: u8 = builder.get_value(cep1155_token, ENABLE_MINT_BURN);
    let transfer_filter_contract: Option<ContractHash> =
        builder.get_value(cep1155_token, TRANSFER_FILTER_CONTRACT);

    assert_eq!(name, TOKEN_NAME);
    assert_eq!(events_mode, 0u8);
    assert_eq!(enable_mint_burn, 0u8);
    assert_eq!(transfer_filter_contract, None);
}

#[test]
fn should_only_allow_init_during_installation_session() {
    let (mut builder, TestContext { cep1155_token: _, .. }) = setup();

    let init_request = ExecuteRequestBuilder::contract_call_by_name(
        *DEFAULT_ACCOUNT_ADDR,
        CEP1155_TEST_TOKEN_CONTRACT_NAME,
        ENTRY_POINT_INIT,
        runtime_args! {},
    )
    .build();
    builder.exec(init_request).expect_failure();

    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep1155Error::ContractAlreadyInitialized as u16,
        "should not allow calls to init() after installation",
    );
}

#[test]
fn should_not_store_balances_or_allowances_under_account_after_install() {
    let (builder, TestContext { cep1155_token: _, .. }) = setup();

    let account = builder
        .get_account(*DEFAULT_ACCOUNT_ADDR)
        .expect("should have account");

    let named_keys = account.named_keys();
    assert!(!named_keys.contains_key(BALANCES), "{:?}", named_keys);
    assert!(!named_keys.contains_key(OPERATORS), "{:?}", named_keys);
}

#[test]
fn should_reject_invalid_collection_name() {
    let mut builder = InMemoryWasmTestBuilder::default();
    builder.run_genesis(&PRODUCTION_RUN_GENESIS_REQUEST);
    let install_request = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        CEP1155_CONTRACT_WASM,
        runtime_args! {
            NAME => 0u64,
        },
    )
    .build();

    builder.exec(install_request).expect_failure();

    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep1155Error::InvalidCollectionName as u16,
        "should not allow calls to init() after installation",
    );
}
