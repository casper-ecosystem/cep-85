use crate::utility::{
    constants::{CEP1155_TEST_TOKEN_CONTRACT_NAME, TOKEN_NAME},
    installer_request_builders::{setup, TestContext},
    support::assert_expected_error,
};
use casper_engine_test_support::{ExecuteRequestBuilder, DEFAULT_ACCOUNT_ADDR};
use casper_types::{runtime_args, RuntimeArgs};
use cep1155::{
    constants::{ENTRY_POINT_INIT, NAME, PACKAGE_HASH},
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

#[test]
fn should_have_queryable_properties() {
    let (mut builder, TestContext { cep1155_token, .. }) = setup();
    let name: String = builder.get_value(cep1155_token, NAME);
    assert_eq!(name, TOKEN_NAME);
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

// TODO
// TOFIX
#[test]
fn should_not_store_balances_or_allowances_under_account_after_install() {
    let (builder, TestContext { cep1155_token: _, .. }) = setup();

    let account = builder
        .get_account(*DEFAULT_ACCOUNT_ADDR)
        .expect("should have account");

    let named_keys = account.named_keys();
    assert!(!named_keys.contains_key(PACKAGE_HASH), "{:?}", named_keys);
}
