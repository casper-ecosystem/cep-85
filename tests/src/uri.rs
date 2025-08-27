use casper_engine_test_support::DEFAULT_ACCOUNT_ADDR;
use casper_types::{runtime_args, EntityAddr, Key, U256};
use cep85::{
    constants::ARG_EVENTS_MODE, error::Cep85Error, events::Uri, modalities::EventsMode,
    utils::replace_token_id_in_uri,
};

use crate::utility::{
    constants::{TOKEN_URI, TOKEN_URI_TEST},
    installer_request_builders::{
        cep85_batch_mint, cep85_check_uri, cep85_mint, cep85_set_uri, setup, setup_with_args,
        TestContext,
    },
    support::{assert_expected_error, get_event},
};

#[test]
fn should_set_specific_uri_on_mint() {
    let (
        mut builder,
        TestContext {
            cep85_contract_hash,
            cep85_test_contract_package,
            ..
        },
    ) = setup();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient = Key::AddressableEntity(EntityAddr::Account(minting_account.value()));
    let mint_amount = U256::from(1);
    let id = U256::one();

    let mint_call = cep85_mint(
        &mut builder,
        &cep85_contract_hash,
        &minting_account,
        &minting_recipient,
        &id,
        &mint_amount,
        Some(TOKEN_URI_TEST),
    );

    mint_call.expect_success().commit();

    let actual_uri = cep85_check_uri(&mut builder, &cep85_test_contract_package, Some(id)).unwrap();
    assert_eq!(actual_uri, replace_token_id_in_uri(TOKEN_URI_TEST, &id));
}

#[test]
fn should_set_specific_uri_on_batch_mint() {
    let (
        mut builder,
        TestContext {
            cep85_contract_hash,
            cep85_test_contract_package,
            ..
        },
    ) = setup();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient = Key::AddressableEntity(EntityAddr::Account(minting_account.value()));
    let ids: Vec<U256> = vec![U256::one(), U256::from(2)];
    let amounts: Vec<U256> = vec![U256::from(2), U256::from(3)];

    // batch_mint is only one recipient
    let mint_call = cep85_batch_mint(
        &mut builder,
        &cep85_contract_hash,
        &minting_account,
        &minting_recipient,
        ids,
        amounts,
        Some(TOKEN_URI_TEST),
    );

    mint_call.expect_success().commit();

    let actual_uri = cep85_check_uri(
        &mut builder,
        &cep85_test_contract_package,
        Some(U256::one()),
    )
    .unwrap();
    assert_eq!(
        actual_uri,
        replace_token_id_in_uri(TOKEN_URI_TEST, &U256::one())
    );
    let actual_uri = cep85_check_uri(
        &mut builder,
        &cep85_test_contract_package,
        Some(U256::from(2)),
    )
    .unwrap();
    assert_eq!(
        actual_uri,
        replace_token_id_in_uri(TOKEN_URI_TEST, &U256::from(2))
    );
}

#[test]
fn should_set_and_get_global_uri() {
    let (
        mut builder,
        TestContext {
            cep85_contract_hash,
            cep85_test_contract_package,
            ..
        },
    ) = setup();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient = Key::AddressableEntity(EntityAddr::Account(minting_account.value()));
    let mint_amount = U256::from(1);
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

    let new_uri = TOKEN_URI_TEST;

    // default address is in admin list, request should succeed
    let updating_account = *DEFAULT_ACCOUNT_ADDR;

    let uri_call = cep85_set_uri(
        &mut builder,
        &cep85_contract_hash,
        &updating_account,
        new_uri,
        None,
    );
    uri_call.expect_success().commit();

    let actual_uri = cep85_check_uri(&mut builder, &cep85_test_contract_package, None).unwrap();
    assert_eq!(actual_uri, TOKEN_URI_TEST);
}

#[test]
fn should_set_and_get_uri_for_id() {
    let (
        mut builder,
        TestContext {
            cep85_contract_hash,
            cep85_test_contract_package,
            ..
        },
    ) = setup();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient = Key::AddressableEntity(EntityAddr::Account(minting_account.value()));
    let mint_amount = U256::from(1);
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

    let new_uri = TOKEN_URI_TEST;

    // default address is in admin list, request should succeed
    let updating_account = *DEFAULT_ACCOUNT_ADDR;

    let uri_call = cep85_set_uri(
        &mut builder,
        &cep85_contract_hash,
        &updating_account,
        new_uri,
        Some(id),
    );

    uri_call.expect_success().commit();

    let actual_uri = cep85_check_uri(&mut builder, &cep85_test_contract_package, Some(id)).unwrap();

    assert_eq!(actual_uri, replace_token_id_in_uri(TOKEN_URI_TEST, &id));
}

#[test]
fn should_fail_to_set_uri_for_non_existing_id() {
    let (
        mut builder,
        TestContext {
            cep85_contract_hash,
            cep85_test_contract_package,
            ..
        },
    ) = setup();

    let id = U256::one();
    let new_uri = TOKEN_URI_TEST;

    // default address is in admin list, request should succeed
    let updating_account = *DEFAULT_ACCOUNT_ADDR;

    let uri_call = cep85_set_uri(
        &mut builder,
        &cep85_contract_hash,
        &updating_account,
        new_uri,
        Some(id),
    );

    uri_call.expect_failure();
    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep85Error::NonSuppliedTokenId as u16,
        "token has no total supply",
    );
    let actual_uri = cep85_check_uri(&mut builder, &cep85_test_contract_package, None).unwrap();
    assert_eq!(actual_uri, TOKEN_URI);
}

#[test]
fn should_fail_to_get_uri_for_non_existing_id() {
    let (
        mut builder,
        TestContext {
            cep85_test_contract_package,
            ..
        },
    ) = setup();

    let id = U256::one();

    let actual_uri = cep85_check_uri(&mut builder, &cep85_test_contract_package, Some(id));

    assert_eq!(actual_uri, None);
}

#[test]
fn should_not_set_empty_global_uri() {
    let (
        mut builder,
        TestContext {
            cep85_contract_hash,
            cep85_test_contract_package,
            ..
        },
    ) = setup();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient = Key::AddressableEntity(EntityAddr::Account(minting_account.value()));
    let mint_amount = U256::from(1);
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

    let new_uri = ""; // Test empty string

    // default address is in admin list, request should succeed
    let updating_account = *DEFAULT_ACCOUNT_ADDR;

    let uri_call = cep85_set_uri(
        &mut builder,
        &cep85_contract_hash,
        &updating_account,
        new_uri,
        None,
    );

    uri_call.expect_failure();
    let error = builder.get_error().expect("must have error");

    assert_expected_error(error, Cep85Error::MissingUri as u16, "empty uri");
    let actual_uri = cep85_check_uri(&mut builder, &cep85_test_contract_package, None).unwrap();
    assert_eq!(actual_uri, TOKEN_URI);

    let new_uri = TOKEN_URI_TEST;

    let uri_call = cep85_set_uri(
        &mut builder,
        &cep85_contract_hash,
        &updating_account,
        new_uri,
        Some(id),
    );

    uri_call.expect_success().commit();

    let actual_uri = cep85_check_uri(&mut builder, &cep85_test_contract_package, Some(id)).unwrap();
    assert_eq!(actual_uri, replace_token_id_in_uri(TOKEN_URI_TEST, &id));
}

#[test]
fn should_set_uri_and_emit_event() {
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
    let minting_recipient = Key::AddressableEntity(EntityAddr::Account(minting_account.value()));
    let mint_amount = U256::from(1);
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

    let new_uri = TOKEN_URI_TEST;

    // default address is in admin list, request should succeed
    let updating_account = *DEFAULT_ACCOUNT_ADDR;

    // Global uri check
    let uri_call = cep85_set_uri(
        &mut builder,
        &cep85_contract_hash,
        &updating_account,
        new_uri,
        None,
    );
    uri_call.expect_success().commit();

    let actual_uri = cep85_check_uri(&mut builder, &cep85_test_contract_package, None).unwrap();
    assert_eq!(actual_uri, TOKEN_URI_TEST);

    // Expect Uri event
    let expected_event = Uri::new(TOKEN_URI_TEST.to_string(), None);
    // Expect event at index 1 (Mint + Uri)
    let event_index = 1;
    let actual_event: Uri = get_event(&mut builder, &cep85_contract_hash, event_index);
    assert_eq!(actual_event, expected_event, "Expected Uri event.");

    // Token uri check
    let uri_call = cep85_set_uri(
        &mut builder,
        &cep85_contract_hash,
        &updating_account,
        new_uri,
        Some(id),
    );
    uri_call.expect_success().commit();

    let actual_uri = cep85_check_uri(&mut builder, &cep85_test_contract_package, Some(id)).unwrap();
    let expected_uri = replace_token_id_in_uri(TOKEN_URI_TEST, &id);
    assert_eq!(actual_uri, expected_uri);

    // Expect Uri event
    let expected_event = Uri::new(TOKEN_URI_TEST.to_string(), Some(id));
    // Expect event at index 2 (Mint + Uri + Uri )
    let event_index = 2;
    let actual_event: Uri = get_event(&mut builder, &cep85_contract_hash, event_index);
    assert_eq!(actual_event, expected_event, "Expected Uri event.");
}
