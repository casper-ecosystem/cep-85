use crate::utility::{
    constants::{CEP1155_CONTRACT_WASM, CEP1155_TEST_TOKEN_CONTRACT_NAME, TOKEN_NAME},
    installer_request_builders::{setup, TestContext},
    support::assert_expected_error,
};
use casper_engine_test_support::{
    ExecuteRequestBuilder, InMemoryWasmTestBuilder, DEFAULT_ACCOUNT_ADDR,
    PRODUCTION_RUN_GENESIS_REQUEST,
};
use casper_execution_engine::core::{engine_state::Error as EngineStateError, execution};
use casper_types::{runtime_args, ApiError, RuntimeArgs};
use cep1155::{
    constants::{BALANCES, ENTRY_POINT_INIT, NAME, PACKAGE_HASH},
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
    assert_eq!(name, TOKEN_NAME);
}

#[test]
fn should_only_allow_init_during_installation_session() {
    let (mut builder, TestContext { cep1155_token, .. }) = setup();

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
    let (builder, TestContext { cep1155_token, .. }) = setup();

    let account = builder
        .get_account(*DEFAULT_ACCOUNT_ADDR)
        .expect("should have account");

    let named_keys = account.named_keys();
    assert!(!named_keys.contains_key(BALANCES), "{:?}", named_keys);
    // TOFIX
    // assert!(!named_keys.contains_key(ALLOWANCE), "{:?}", named_keys);
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

    let error = ApiError::InvalidArgument;
    let expected_error = EngineStateError::Exec(execution::Error::Revert(error));
    let actual_error = builder.get_error().expect("must have error");

    if let EngineStateError::Exec(execution::Error::Revert(ref api_error)) = actual_error {
        if api_error == &error {
            return;
        }
    }

    panic!("Expected {:?}, received {:?}", expected_error, actual_error);
}
