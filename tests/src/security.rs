use std::collections::HashMap;

use casper_engine_test_support::{ExecuteRequestBuilder, ARG_AMOUNT};
use casper_types::{runtime_args, system::mint::ARG_ID, Key, RuntimeArgs, U256};
use cep85::{
    constants::{
        ARG_ENABLE_MINT_BURN, ARG_EVENTS_MODE, ARG_NAME, ARG_OWNER, ARG_RECIPIENT, ARG_URI,
        ENTRY_POINT_BURN, ENTRY_POINT_MINT, MINTER_LIST, TOKEN_URI,
    },
    error::Cep85Error,
    modalities::EventsMode,
};

use crate::utility::{
    constants::{ACCOUNT_USER_1, ACCOUNT_USER_2, TOKEN_NAME},
    installer_request_builders::{cep85_mint, setup_with_args, TestContext},
    support::{assert_expected_error, create_dummy_key_pair, fund_account},
};

#[test]
fn test_security_no_rights() {
    let (
        mut builder,
        TestContext {
            cep85_token,
            test_accounts,
            ..
        },
    ) = setup_with_args(
        runtime_args! {
            ARG_NAME => TOKEN_NAME,
            ARG_URI => TOKEN_URI,
            ARG_EVENTS_MODE => EventsMode::CES as u8,
            ARG_ENABLE_MINT_BURN => true
        },
        None,
    );

    let account_user_1 = *test_accounts.get(&ACCOUNT_USER_1).unwrap();
    let recipient: Key = account_user_1.into();

    let mint_amount = U256::one();
    let id = U256::one();

    let failing_mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        account_user_1,
        cep85_token,
        ENTRY_POINT_MINT,
        runtime_args! {
            ARG_RECIPIENT => recipient,
            ARG_ID => id,
            ARG_AMOUNT => mint_amount,
        },
    )
    .build();
    builder.exec(failing_mint_request).expect_failure();

    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep85Error::InsufficientRights as u16,
        "should not allow to mint for non default admin account",
    );

    cep85_mint(&mut builder, &cep85_token, recipient, id, mint_amount);

    // New owner is now ACCOUNT_USER_1
    let owner = recipient;

    let burn_request = ExecuteRequestBuilder::contract_call_by_hash(
        account_user_1,
        cep85_token,
        ENTRY_POINT_BURN,
        runtime_args! {
            ARG_OWNER => owner,
            ARG_ID => id,
            ARG_AMOUNT => mint_amount,
        },
    )
    .build();

    builder.exec(burn_request).expect_success().commit();
}

#[test]
fn test_security_minter_rights() {
    let (_, public_key_account_user_1) = create_dummy_key_pair(ACCOUNT_USER_1);
    let account_user_1 = public_key_account_user_1.to_account_hash();
    let mut test_accounts = HashMap::new();
    test_accounts.insert(ACCOUNT_USER_1, account_user_1);

    let (
        mut builder,
        TestContext {
            cep85_token,
            test_accounts,
            ..
        },
    ) = setup_with_args(
        runtime_args! {
            ARG_NAME => TOKEN_NAME,
            ARG_URI => TOKEN_URI,
            ARG_EVENTS_MODE => EventsMode::CES as u8,
            ARG_ENABLE_MINT_BURN => true,
            MINTER_LIST => vec![Key::Account(account_user_1)]
        },
        Some(test_accounts),
    );

    // account_user_1 was created before genesis and is not yet funded so fund it
    fund_account(&mut builder, account_user_1);

    let mint_amount = U256::one();
    let id = U256::one();

    let mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        account_user_1,
        cep85_token,
        ENTRY_POINT_MINT,
        runtime_args! {
            ARG_RECIPIENT => Key::from(account_user_1),
            ARG_ID => id,
            ARG_AMOUNT => mint_amount,
        },
    )
    .build();

    builder.exec(mint_request).commit().expect_success();

    // account_user_2 is not in minter list, request should fail
    let account_user_2 = *test_accounts.get(&ACCOUNT_USER_2).unwrap();
    let failing_mint_request = ExecuteRequestBuilder::contract_call_by_hash(
        account_user_2,
        cep85_token,
        ENTRY_POINT_MINT,
        runtime_args! {
            ARG_RECIPIENT => Key::from(account_user_2),
            ARG_ID => id,
            ARG_AMOUNT => mint_amount,
        },
    )
    .build();

    builder.exec(failing_mint_request).commit().expect_failure();
}
