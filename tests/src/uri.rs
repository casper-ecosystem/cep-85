use casper_engine_test_support::DEFAULT_ACCOUNT_ADDR;
use casper_types::{runtime_args, Key, RuntimeArgs, U256};
use cep85::{
    constants::{ARG_ENABLE_MINT_BURN, ARG_EVENTS_MODE, ARG_NAME, ARG_URI},
    modalities::EventsMode,
    utils::replace_token_id_in_uri,
};

use crate::utility::{
    constants::{TOKEN_NAME, TOKEN_URI, TOKEN_URI_TEST},
    installer_request_builders::{
        cep85_check_uri, cep85_mint, cep85_set_uri, setup_with_args, TestContext,
    },
};

#[test]
fn should_set_global_uri() {
    let (
        mut builder,
        TestContext {
            cep85_token,
            cep85_test_contract_package,
            ..
        },
    ) = setup_with_args(
        runtime_args! {
            ARG_NAME => TOKEN_NAME,
            ARG_URI => TOKEN_URI,
            ARG_EVENTS_MODE => EventsMode::CES as u8,
            ARG_ENABLE_MINT_BURN => true,
        },
        None,
    );

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let recipient: Key = minting_account.into();
    let mint_amount = U256::from(1);
    let id = U256::one();

    let mint_call = cep85_mint(
        &mut builder,
        &cep85_token,
        minting_account,
        recipient,
        id,
        mint_amount,
    );

    mint_call.expect_success().commit();

    let new_uri = TOKEN_URI_TEST;

    // default address is in admin list, request should succeed
    let updating_account = *DEFAULT_ACCOUNT_ADDR;

    let meta_call = cep85_set_uri(&mut builder, &cep85_token, updating_account, new_uri, None);
    meta_call.expect_success().commit();

    let actual_uri = cep85_check_uri(&mut builder, &cep85_test_contract_package, None);

    assert_eq!(actual_uri, TOKEN_URI_TEST);
}

#[test]
fn should_set_uri_for_id() {
    let (
        mut builder,
        TestContext {
            cep85_token,
            cep85_test_contract_package,
            ..
        },
    ) = setup_with_args(
        runtime_args! {
            ARG_NAME => TOKEN_NAME,
            ARG_URI => TOKEN_URI,
            ARG_EVENTS_MODE => EventsMode::CES as u8,
            ARG_ENABLE_MINT_BURN => true,
        },
        None,
    );

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let recipient: Key = minting_account.into();
    let mint_amount = U256::from(1);
    let id = U256::one();

    let mint_call = cep85_mint(
        &mut builder,
        &cep85_token,
        minting_account,
        recipient,
        id,
        mint_amount,
    );

    mint_call.expect_success().commit();

    let new_uri = TOKEN_URI_TEST;

    // default address is in admin list, request should succeed
    let updating_account = *DEFAULT_ACCOUNT_ADDR;

    let meta_call = cep85_set_uri(
        &mut builder,
        &cep85_token,
        updating_account,
        new_uri,
        Some(id),
    );

    meta_call.expect_success().commit();

    let actual_uri = cep85_check_uri(&mut builder, &cep85_test_contract_package, Some(id));

    assert_eq!(actual_uri, replace_token_id_in_uri(TOKEN_URI_TEST, &id));
}
