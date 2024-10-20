use crate::utility::{
    constants::TOKEN_URI,
    installer_request_builders::{
        cep85_check_balance_of, cep85_mint, setup, setup_with_args, TestContext,
    },
    support::{get_dictionary_value_from_key, get_event, get_test_account},
};
use casper_engine_test_support::DEFAULT_ACCOUNT_ADDR;
use casper_event_standard::{Schemas, EVENTS_DICT, EVENTS_SCHEMA};
use casper_types::{contract_messages::Message, runtime_args, EntityAddr, Key, U256};
use cep85::{
    constants::{ARG_EVENTS_MODE, EVENTS},
    events::{
        ApprovalForAll, Burn, BurnBatch, ChangeEnableBurnMode, ChangeEventsMode, ChangeSecurity,
        Mint, MintBatch, SetModalities, SetTotalSupply, Transfer, TransferBatch, Upgrade, Uri,
        UriBatch,
    },
    modalities::EventsMode,
};

#[test]
fn should_have_events_schema_in_events_mode() {
    let (
        mut builder,
        TestContext {
            cep85_contract_hash,
            ..
        },
    ) = setup_with_args(runtime_args! {
        ARG_EVENTS_MODE => EventsMode::CES as u8,
    });
    let expected_schemas = Schemas::new()
        .with::<Mint>()
        .with::<MintBatch>()
        .with::<Burn>()
        .with::<BurnBatch>()
        .with::<ApprovalForAll>()
        .with::<Transfer>()
        .with::<TransferBatch>()
        .with::<Uri>()
        .with::<UriBatch>()
        .with::<SetTotalSupply>()
        .with::<ChangeSecurity>()
        .with::<SetModalities>()
        .with::<Upgrade>()
        .with::<ChangeEventsMode>()
        .with::<ChangeEnableBurnMode>();
    let contract_entity_addr = EntityAddr::new_smart_contract(cep85_contract_hash.value());
    let actual_schemas: Schemas = builder.get_value(contract_entity_addr, EVENTS_SCHEMA);
    assert_eq!(actual_schemas, expected_schemas, "Schemas mismatch.");
}

#[test]
fn should_not_have_events_dict_in_no_events_mode() {
    let (
        builder,
        TestContext {
            cep85_contract_hash,
            ..
        },
    ) = setup();

    let contract = builder
        .get_entity_with_named_keys_by_entity_hash(cep85_contract_hash)
        .expect("should have contract");
    let named_keys = contract.named_keys();
    let events = named_keys.get(EVENTS_DICT);
    assert_eq!(events, None);
}

#[test]
fn should_have_events_dict_with_events_mode_ces() {
    let (
        builder,
        TestContext {
            cep85_contract_hash,
            ..
        },
    ) = setup_with_args(runtime_args! {
        ARG_EVENTS_MODE => EventsMode::CES as u8,
    });

    let contract = builder
        .get_entity_with_named_keys_by_entity_hash(cep85_contract_hash)
        .expect("should have contract");
    let named_keys = contract.named_keys();
    let events = named_keys.get(EVENTS_DICT);
    assert!(events.is_some());
}

#[test]
#[should_panic]
fn should_not_record_events_in_no_events_mode() {
    let (account_user_1_key, _, _) = get_test_account("ACCOUNT_USER_1");

    let (
        mut builder,
        TestContext {
            cep85_contract_hash,
            cep85_test_contract_package,
            ..
        },
    ) = setup();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient = account_user_1_key;
    let mint_amount = U256::one();
    let id = U256::one();

    let mint_call = cep85_mint(
        &mut builder,
        &cep85_contract_hash,
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
    )
    .unwrap();
    let expected_balance = U256::one();

    assert_eq!(actual_balance, expected_balance);

    // Query for the Mint event here and expect failure
    // as no events are being written to global state.
    get_dictionary_value_from_key::<()>(&mut builder, &cep85_contract_hash, EVENTS_DICT, "1");
}

#[test]
fn should_record_events_in_ces_events_mode() {
    let (account_user_1_key, _, _) = get_test_account("ACCOUNT_USER_1");

    let (
        mut builder,
        TestContext {
            cep85_contract_hash,
            cep85_test_contract_package,
            ..
        },
    ) = setup_with_args(runtime_args! {
        ARG_EVENTS_MODE => EventsMode::CES as u8,
    });

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient = account_user_1_key;
    let mint_amount = U256::one();
    let id = U256::one();

    let mint_call = cep85_mint(
        &mut builder,
        &cep85_contract_hash,
        &minting_account,
        &minting_recipient,
        &id,
        &mint_amount,
        Some(TOKEN_URI),
    );

    mint_call.expect_success().commit();

    let actual_balance = cep85_check_balance_of(
        &mut builder,
        &cep85_test_contract_package,
        &minting_recipient,
        &id,
    )
    .unwrap();
    let expected_balance = U256::one();

    assert_eq!(actual_balance, expected_balance);

    // Expect Mint event
    let expected_event = Mint::new(id, minting_recipient, mint_amount);
    let actual_event: Mint = get_event(&mut builder, &cep85_contract_hash, 0);
    assert_eq!(actual_event, expected_event, "Expected Mint event.");

    // Expect Uri event
    let expected_event = Uri::new(TOKEN_URI.to_string(), Some(id));
    let actual_event: Uri = get_event(&mut builder, &cep85_contract_hash, 1);
    assert_eq!(actual_event, expected_event, "Expected Uri event.");
}

#[test]
fn should_record_events_in_native_events_mode() {
    let (account_user_1_key, account_user_1_account_hash, _) = get_test_account("ACCOUNT_USER_1");

    let (
        mut builder,
        TestContext {
            cep85_contract_hash,
            cep85_contract_key,
            cep85_test_contract_package,
            ..
        },
    ) = setup_with_args(runtime_args! {
        ARG_EVENTS_MODE => EventsMode::Native as u8,
    });

    let binding = builder.query(None, cep85_contract_key, &[]).unwrap();
    let entity = binding.as_addressable_entity().unwrap();

    let (topic_name, message_topic_hash) = entity
        .message_topics()
        .iter()
        .next()
        .expect("should have at least one topic");

    assert_eq!(topic_name, &EVENTS.to_string());

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient = account_user_1_key;
    let mint_amount = U256::one();
    let id = U256::one();

    let mint_call = cep85_mint(
        &mut builder,
        &cep85_contract_hash,
        &minting_account,
        &minting_recipient,
        &id,
        &mint_amount,
        Some(TOKEN_URI),
    );

    mint_call.expect_success().commit();

    let actual_balance = cep85_check_balance_of(
        &mut builder,
        &cep85_test_contract_package,
        &minting_recipient,
        &id,
    )
    .unwrap();
    let expected_balance = U256::one();

    assert_eq!(actual_balance, expected_balance);

    let binding = builder
        .query(
            None,
            Key::message_topic(entity.entity_addr(cep85_contract_hash), *message_topic_hash),
            &[],
        )
        .unwrap();

    let message_topic_summary = binding.as_message_topic_summary().unwrap();
    let message_count = message_topic_summary.message_count();

    let expected_message_count = 2_u32; // Mint +Uri

    assert_eq!(message_count, expected_message_count);

    let exec_result = builder.get_exec_result_owned(2).unwrap();
    let messages = exec_result.messages();

    let mint_message = &format!("Mint(Mint {{ id: 1, recipient: Key::AddressableEntity(account-{account_user_1_account_hash}), amount: 1 }})");
    let mint_message = Message::new(
        entity.entity_addr(cep85_contract_hash),
        mint_message.into(),
        EVENTS.to_string(),
        *message_topic_hash,
        0,
        0,
    );

    let uri_message = &format!("Uri(Uri {{ value: \"{TOKEN_URI}\", id: Some({id}) }})");
    let uri_message = Message::new(
        entity.entity_addr(cep85_contract_hash),
        uri_message.into(),
        EVENTS.to_string(),
        *message_topic_hash,
        1,
        1,
    );

    assert_eq!(messages, &vec![mint_message, uri_message]);
}

#[test]
fn should_record_events_in_native_and_ces_events_mode() {
    let (account_user_1_key, account_user_1_account_hash, _) = get_test_account("ACCOUNT_USER_1");

    let (
        mut builder,
        TestContext {
            cep85_contract_hash,
            cep85_contract_key,
            cep85_test_contract_package,
            ..
        },
    ) = setup_with_args(runtime_args! {
        ARG_EVENTS_MODE => EventsMode::NativeNCES as u8,
    });

    let binding = builder.query(None, cep85_contract_key, &[]).unwrap();
    let entity = binding.as_addressable_entity().unwrap();

    let (topic_name, message_topic_hash) = entity
        .message_topics()
        .iter()
        .next()
        .expect("should have at least one topic");

    assert_eq!(topic_name, &EVENTS.to_string());

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient = account_user_1_key;
    let mint_amount = U256::one();
    let id = U256::one();

    let mint_call = cep85_mint(
        &mut builder,
        &cep85_contract_hash,
        &minting_account,
        &minting_recipient,
        &id,
        &mint_amount,
        Some(TOKEN_URI),
    );

    mint_call.expect_success().commit();

    let actual_balance = cep85_check_balance_of(
        &mut builder,
        &cep85_test_contract_package,
        &minting_recipient,
        &id,
    )
    .unwrap();
    let expected_balance = U256::one();

    assert_eq!(actual_balance, expected_balance);

    let binding = builder
        .query(
            None,
            Key::message_topic(entity.entity_addr(cep85_contract_hash), *message_topic_hash),
            &[],
        )
        .unwrap();

    let message_topic_summary = binding.as_message_topic_summary().unwrap();
    let message_count = message_topic_summary.message_count();

    let expected_message_count = 2_u32; // Mint +Uri

    assert_eq!(message_count, expected_message_count);

    // Native events
    let exec_result = builder.get_exec_result_owned(2).unwrap();
    let messages = exec_result.messages();
    let mint_message = &format!("Mint(Mint {{ id: 1, recipient: Key::AddressableEntity(account-{account_user_1_account_hash}), amount: 1 }})");
    let mint_message = Message::new(
        entity.entity_addr(cep85_contract_hash),
        mint_message.into(),
        EVENTS.to_string(),
        *message_topic_hash,
        0,
        0,
    );

    let uri_message = &format!("Uri(Uri {{ value: \"{TOKEN_URI}\", id: Some({id}) }})");
    let uri_message = Message::new(
        entity.entity_addr(cep85_contract_hash),
        uri_message.into(),
        EVENTS.to_string(),
        *message_topic_hash,
        1,
        1,
    );

    assert_eq!(messages, &vec![mint_message, uri_message]);

    // CES events
    // Expect Mint event
    let expected_event = Mint::new(id, minting_recipient, mint_amount);
    let actual_event: Mint = get_event(&mut builder, &cep85_contract_hash, 0);
    assert_eq!(actual_event, expected_event, "Expected Mint event.");

    // Expect Uri event
    let expected_event = Uri::new(TOKEN_URI.to_string(), Some(id));
    let actual_event: Uri = get_event(&mut builder, &cep85_contract_hash, 1);
    assert_eq!(actual_event, expected_event, "Expected Uri event.");
}
