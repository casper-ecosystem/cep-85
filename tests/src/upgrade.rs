use crate::utility::{
    constants::{
        CEP85_CONTRACT_WASM, CEP85_TEST_TOKEN_CONTRACT_NAME, CEP85_TEST_TOKEN_CONTRACT_VERSION,
        TOKEN_NAME,
    },
    installer_request_builders::{setup, setup_with_args, TestContext},
    support::get_event,
};
use casper_engine_test_support::{ExecuteRequestBuilder, DEFAULT_ACCOUNT_ADDR};
use casper_types::{runtime_args, AddressableEntityHash, Key};
use cep85::{
    constants::{ARG_CONTRACT_HASH, ARG_EVENTS_MODE, ARG_NAME, ARG_UPGRADE_FLAG},
    events::Upgrade,
    modalities::EventsMode,
};

#[test]
fn should_upgrade_and_update_account_contract_contexts() {
    let (
        mut builder,
        TestContext {
            cep85_contract_hash,
            ..
        },
    ) = setup();

    let contract = builder
        .get_entity_with_named_keys_by_entity_hash(cep85_contract_hash)
        .expect("should have contract");

    let cep85_contract_hash_contract_version = builder
        .query(
            None,
            Key::Account(*DEFAULT_ACCOUNT_ADDR),
            &[CEP85_TEST_TOKEN_CONTRACT_VERSION.to_string()],
        )
        .unwrap()
        .as_cl_value()
        .unwrap()
        .to_owned()
        .into_t::<u32>()
        .unwrap();

    assert_eq!(cep85_contract_hash_contract_version, 1_u32);

    let contract_hash_on_install: AddressableEntityHash = contract
        .named_keys()
        .get(ARG_CONTRACT_HASH)
        .expect("should have contract hash")
        .into_entity_hash()
        .unwrap();

    let upgrade_request_contract = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        CEP85_CONTRACT_WASM,
        runtime_args! {
            ARG_UPGRADE_FLAG => true,
            ARG_NAME => TOKEN_NAME,
        },
    )
    .build();

    builder
        .exec(upgrade_request_contract)
        .expect_success()
        .commit();

    let account = builder
        .get_entity_with_named_keys_by_account_hash(*DEFAULT_ACCOUNT_ADDR)
        .expect("should have account");

    let upgraded_cep85_contract_hash: AddressableEntityHash = account
        .named_keys()
        .get(CEP85_TEST_TOKEN_CONTRACT_NAME)
        .unwrap()
        .into_entity_hash()
        .unwrap();

    let contract = builder
        .get_entity_with_named_keys_by_entity_hash(upgraded_cep85_contract_hash)
        .expect("should have contract");

    let contract_hash_after_upgrade: AddressableEntityHash = contract
        .named_keys()
        .get(ARG_CONTRACT_HASH)
        .unwrap()
        .into_entity_hash()
        .unwrap();

    assert_ne!(
        contract_hash_on_install.to_formatted_string(),
        contract_hash_after_upgrade.to_formatted_string()
    );

    let cep85_contract_hash_contract_version = builder
        .query(
            None,
            Key::Account(*DEFAULT_ACCOUNT_ADDR),
            &[CEP85_TEST_TOKEN_CONTRACT_VERSION.to_string()],
        )
        .unwrap()
        .as_cl_value()
        .unwrap()
        .to_owned()
        .into_t::<u32>()
        .unwrap();

    assert_eq!(cep85_contract_hash_contract_version, 2_u32);
}

#[test]
fn should_emit_event_on_upgrade_with_events_mode_ces() {
    let (
        mut builder,
        TestContext {
            cep85_contract_hash,
            ..
        },
    ) = setup_with_args(runtime_args! {
        ARG_EVENTS_MODE => EventsMode::CES as u8,
    });

    let upgrade_request_contract = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        CEP85_CONTRACT_WASM,
        runtime_args! {
            ARG_UPGRADE_FLAG => true,
            ARG_NAME => TOKEN_NAME,
        },
    )
    .build();
    builder
        .exec(upgrade_request_contract)
        .expect_success()
        .commit();

    // Expect Upgrade event
    let expected_event = Upgrade::new();
    let actual_event: Upgrade = get_event(&mut builder, &cep85_contract_hash, 0);
    assert_eq!(actual_event, expected_event, "Expected Upgrade event.");
}
