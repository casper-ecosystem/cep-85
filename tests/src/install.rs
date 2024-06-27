use crate::utility::{
    constants::{CEP85_CONTRACT_WASM, CEP85_TEST_TOKEN_CONTRACT_NAME, TOKEN_NAME, TOKEN_URI},
    installer_request_builders::{setup, TestContext},
    support::assert_expected_error,
};
use casper_engine_test_support::{
    utils::create_run_genesis_request, ExecuteRequestBuilder, LmdbWasmTestBuilder,
    DEFAULT_ACCOUNT_ADDR, DEFAULT_ACCOUNT_PUBLIC_KEY,
};
use casper_types::{runtime_args, AddressableEntityHash, EntityAddr, GenesisAccount, Motes, U512};
use cep85::{
    constants::{
        ARG_ENABLE_BURN, ARG_EVENTS_MODE, ARG_NAME, ARG_PACKAGE_HASH, ARG_TRANSFER_FILTER_CONTRACT,
        ARG_TRANSFER_FILTER_METHOD, ARG_URI, DICT_BALANCES, DICT_OPERATORS, ENTRY_POINT_INIT,
    },
    error::Cep85Error,
    modalities::EventsMode,
};

#[test]
fn should_install_contract() {
    let (builder, TestContext { cep18_contract_hash, .. }) = setup();
    let contract = builder
        .get_entity_with_named_keys_by_entity_hash(cep18_contract_hash)
        .expect("should have contract");
    let named_keys = contract.named_keys();
    assert!(named_keys.contains(ARG_PACKAGE_HASH), "{:?}", named_keys);
}

#[test]
fn should_have_queryable_properties() {
    let (mut builder, TestContext { cep18_contract_hash, .. }) = setup();
    let contract_entity_addr = EntityAddr::new_smart_contract(cep18_contract_hash.value());
    let name: String = builder.get_value(contract_entity_addr, ARG_NAME);
    let uri: String = builder.get_value(contract_entity_addr, ARG_URI);
    let events_mode: u8 = builder.get_value(contract_entity_addr, ARG_EVENTS_MODE);
    let enable_burn: bool = builder.get_value(contract_entity_addr, ARG_ENABLE_BURN);
    let transfer_filter_contract: Option<AddressableEntityHash> =
        builder.get_value(contract_entity_addr, ARG_TRANSFER_FILTER_CONTRACT);
    let transfer_filter_method: Option<String> =
        builder.get_value(contract_entity_addr, ARG_TRANSFER_FILTER_METHOD);

    assert_eq!(name, TOKEN_NAME);
    assert_eq!(uri, TOKEN_URI);
    assert_eq!(events_mode, EventsMode::NoEvents as u8);
    assert!(!enable_burn);
    assert_eq!(transfer_filter_contract, None);
    assert_eq!(transfer_filter_method, None);
}

#[test]
fn should_only_allow_init_during_installation_session() {
    let (mut builder, TestContext { cep18_contract_hash: _, .. }) = setup();

    let init_request = ExecuteRequestBuilder::contract_call_by_name(
        *DEFAULT_ACCOUNT_ADDR,
        CEP85_TEST_TOKEN_CONTRACT_NAME,
        ENTRY_POINT_INIT,
        runtime_args! {},
    )
    .build();
    builder.exec(init_request).expect_failure();

    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep85Error::ContractAlreadyInitialized as u16,
        "should not allow calls to init() after installation",
    );
}

#[test]
fn should_not_store_balances_or_allowances_under_account_after_install() {
    let (builder, TestContext { cep18_contract_hash: _, .. }) = setup();

    let account = builder
        .get_entity_with_named_keys_by_account_hash(*DEFAULT_ACCOUNT_ADDR)
        .expect("should have account");

    let named_keys = account.named_keys();
    assert!(!named_keys.contains(DICT_BALANCES), "{:?}", named_keys);
    assert!(!named_keys.contains(DICT_OPERATORS), "{:?}", named_keys);
}

#[test]
fn should_reject_invalid_collection_name() {
    let mut builder = LmdbWasmTestBuilder::default();
    builder
        .run_genesis(create_run_genesis_request(vec![GenesisAccount::Account {
            public_key: DEFAULT_ACCOUNT_PUBLIC_KEY.clone(),
            balance: Motes::new(U512::from(5_000_000_000_000_u64)),
            validator: None,
        }]))
        .commit();
    let install_request = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        CEP85_CONTRACT_WASM,
        runtime_args! {
            ARG_NAME => 0_u64,
            ARG_URI => TOKEN_URI,
        },
    )
    .build();

    builder.exec(install_request).expect_failure();

    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep85Error::InvalidCollectionName as u16,
        "should not allow calls to init() after installation",
    );
}
