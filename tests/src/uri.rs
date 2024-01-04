use casper_engine_test_support::{ExecuteRequestBuilder, DEFAULT_ACCOUNT_ADDR};
use casper_types::{runtime_args, Key, RuntimeArgs, U256};
use cep85::{constants::ARG_ID, error::Cep85Error, utils::replace_token_id_in_uri};
use cep85_test_contract::constants::ENTRY_POINT_CHECK_URI;

use crate::utility::{
    constants::TOKEN_URI_TEST,
    installer_request_builders::{cep85_check_uri, cep85_mint, cep85_set_uri, setup, TestContext},
    support::assert_expected_error,
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
    ) = setup();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient: Key = minting_account.into();
    let mint_amount = U256::from(1);
    let id = U256::one();

    let mint_call = cep85_mint(
        &mut builder,
        &cep85_token,
        &minting_account,
        &minting_recipient,
        &id,
        &mint_amount,
    );

    mint_call.expect_success().commit();

    let new_uri = TOKEN_URI_TEST;

    // default address is in admin list, request should succeed
    let updating_account = *DEFAULT_ACCOUNT_ADDR;

    let uri_call = cep85_set_uri(&mut builder, &cep85_token, &updating_account, new_uri, None);
    uri_call.expect_success().commit();

    let actual_uri = cep85_check_uri(&mut builder, &cep85_test_contract_package, None);
    assert_eq!(actual_uri, TOKEN_URI_TEST);
}

#[test]
fn should_get_uri_for_id() {
    let (
        mut builder,
        TestContext {
            cep85_token,
            cep85_test_contract_package,
            ..
        },
    ) = setup();

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let minting_recipient: Key = minting_account.into();
    let mint_amount = U256::from(1);
    let id = U256::one();

    let mint_call = cep85_mint(
        &mut builder,
        &cep85_token,
        &minting_account,
        &minting_recipient,
        &id,
        &mint_amount,
    );

    mint_call.expect_success().commit();

    let new_uri = TOKEN_URI_TEST;

    // default address is in admin list, request should succeed
    let updating_account = *DEFAULT_ACCOUNT_ADDR;

    let uri_call = cep85_set_uri(
        &mut builder,
        &cep85_token,
        &updating_account,
        new_uri,
        Some(id),
    );

    uri_call.expect_success().commit();

    let actual_uri = cep85_check_uri(&mut builder, &cep85_test_contract_package, Some(id));

    assert_eq!(actual_uri, replace_token_id_in_uri(TOKEN_URI_TEST, &id));
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

    let exec_request = ExecuteRequestBuilder::versioned_contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        cep85_test_contract_package,
        None,
        ENTRY_POINT_CHECK_URI,
        runtime_args! {
            ARG_ID => Some(id),
        },
    )
    .build();
    builder.exec(exec_request).expect_failure();

    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep85Error::MissingUri as u16,
        "non existing token has no uri",
    );
}
