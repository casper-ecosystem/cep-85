use crate::utility::{
    constants::{ACCOUNT_USER_1, TOKEN_URI},
    installer_request_builders::{
        cep85_check_balance_of, cep85_mint, setup, setup_with_args, TestContext,
    },
    support::{get_dictionary_value_from_key, get_event},
};
use casper_engine_test_support::DEFAULT_ACCOUNT_ADDR;
use casper_event_standard::{Schemas, EVENTS_DICT, EVENTS_SCHEMA};
use casper_types::{runtime_args, Key, RuntimeArgs, U256};
use cep85::{
    constants::ARG_EVENTS_MODE,
    events::{
        ApprovalForAll, Burn, ChangeSecurity, Mint, SetModalities, SetTotalSupply, TransferBatch,
        TransferSingle, Upgrade, Uri,
    },
    modalities::EventsMode,
};

#[test]
fn should_have_events_schema_in_events_mode() {
    let (mut builder, TestContext { cep85_token, .. }) = setup_with_args(
        runtime_args! {
            ARG_EVENTS_MODE => EventsMode::CES as u8,
        },
        None,
    );
    let expected_schemas = Schemas::new()
        .with::<Mint>()
        .with::<Burn>()
        .with::<ApprovalForAll>()
        .with::<TransferSingle>()
        .with::<TransferBatch>()
        .with::<Uri>()
        .with::<SetTotalSupply>()
        .with::<ChangeSecurity>()
        .with::<SetModalities>()
        .with::<Upgrade>();
    let actual_schemas: Schemas = builder.get_value(cep85_token, EVENTS_SCHEMA);
    assert_eq!(actual_schemas, expected_schemas, "Schemas mismatch.");
}

#[test]
fn should_not_have_events_dict_in_no_events_mode() {
    let (builder, TestContext { cep85_token, .. }) = setup();

    let contract = builder
        .get_contract(cep85_token)
        .expect("should have contract");
    let named_keys = contract.named_keys();
    let events = named_keys.get(EVENTS_DICT);
    assert_eq!(events, None);
}

#[test]
fn should_have_events_dict_with_events_mode_ces() {
    let (builder, TestContext { cep85_token, .. }) = setup_with_args(
        runtime_args! {
            ARG_EVENTS_MODE => EventsMode::CES as u8,
        },
        None,
    );

    let contract = builder
        .get_contract(cep85_token)
        .expect("should have contract");
    let named_keys = contract.named_keys();
    let events = named_keys.get(EVENTS_DICT);
    assert!(events.is_some());
}

#[test]
fn should_record_events_in_events_mode() {
    let (
        mut builder,
        TestContext {
            cep85_token,
            cep85_test_contract_package,
            ref test_accounts,
            ..
        },
    ) = setup_with_args(
        runtime_args! {
            ARG_EVENTS_MODE => EventsMode::CES as u8,
        },
        None,
    );

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient: Key = Key::from(*test_accounts.get(&ACCOUNT_USER_1).unwrap());
    let mint_amount = U256::one();
    let id = U256::one();

    let mint_call = cep85_mint(
        &mut builder,
        &cep85_token,
        &minting_account,
        &minting_recipient,
        &id,
        &mint_amount,
        None,
    );

    mint_call.expect_success().commit();

    let actual_balance = cep85_check_balance_of(
        &mut builder,
        &cep85_test_contract_package,
        &minting_recipient,
        &id,
    );
    let expected_balance = U256::one();

    assert_eq!(actual_balance, expected_balance);

    // Expect Mint event
    let expected_event = Mint::new(id, minting_recipient, mint_amount);
    let actual_event: Mint = get_event(&builder, &cep85_token.into(), 0);
    assert_eq!(actual_event, expected_event, "Expected Mint event.");

    // Expect Uri event
    let expected_event = Uri::new(TOKEN_URI.into(), Some(id));
    let actual_event: Uri = get_event(&builder, &cep85_token.into(), 1);
    assert_eq!(actual_event, expected_event, "Expected Uri event.");
}

#[test]
#[should_panic]
fn should_not_record_events_in_no_events_mode() {
    let (
        mut builder,
        TestContext {
            cep85_token,
            cep85_test_contract_package,
            ref test_accounts,
            ..
        },
    ) = setup();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient: Key = Key::from(*test_accounts.get(&ACCOUNT_USER_1).unwrap());
    let mint_amount = U256::one();
    let id = U256::one();

    let mint_call = cep85_mint(
        &mut builder,
        &cep85_token,
        &minting_account,
        &minting_recipient,
        &id,
        &mint_amount,
        None,
    );

    mint_call.expect_success().commit();

    let actual_balance = cep85_check_balance_of(
        &mut builder,
        &cep85_test_contract_package,
        &minting_recipient,
        &id,
    );
    let expected_balance = U256::one();

    assert_eq!(actual_balance, expected_balance);

    // Query for the Mint event here and expect failure
    // as no events are being written to global state.
    get_dictionary_value_from_key::<()>(&builder, &cep85_token.into(), EVENTS_DICT, "1");
}
