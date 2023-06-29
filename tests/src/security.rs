use std::collections::HashMap;

use casper_engine_test_support::DEFAULT_ACCOUNT_ADDR;
use casper_types::{runtime_args, Key, RuntimeArgs, U256};
use cep85::{
    constants::{
        ADMIN_LIST, ARG_ENABLE_MINT_BURN, ARG_EVENTS_MODE, ARG_NAME, ARG_URI, BURNER_LIST,
        MINTER_LIST, TOKEN_URI,
    },
    error::Cep85Error,
    modalities::EventsMode,
};

use crate::utility::{
    constants::{ACCOUNT_USER_1, ACCOUNT_USER_2, TOKEN_NAME},
    installer_request_builders::{
        cep85_burn, cep85_change_security, cep85_mint, cep85_set_total_supply_of, setup_with_args,
        SecurityLists, TestContext,
    },
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

    let minting_account = *test_accounts.get(&ACCOUNT_USER_1).unwrap();
    let recipient: Key = minting_account.into();

    let mint_amount = U256::one();
    let id = U256::one();

    let failing_mint_call = cep85_mint(
        &mut builder,
        &cep85_token,
        minting_account,
        recipient,
        id,
        mint_amount,
    );

    // minting account is not in minter nor admin list
    failing_mint_call.expect_failure();

    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep85Error::InsufficientRights as u16,
        "should not allow to mint for non default admin account",
    );

    let minting_account = *DEFAULT_ACCOUNT_ADDR;

    let mint_call = cep85_mint(
        &mut builder,
        &cep85_token,
        minting_account,
        recipient,
        id,
        mint_amount,
    );

    // default address is in admin list by default
    mint_call.expect_success().commit();

    // New owner is now ACCOUNT_USER_1 but is not in
    let bunrning_account = *test_accounts.get(&ACCOUNT_USER_1).unwrap();
    let owner: Key = bunrning_account.into();

    let burn_call = cep85_burn(
        &mut builder,
        &cep85_token,
        bunrning_account,
        owner,
        id,
        mint_amount,
    );

    burn_call.expect_failure();

    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep85Error::InsufficientRights as u16,
        "should not allow to mint for non default admin account",
    );
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
            MINTER_LIST => vec![Key::from(account_user_1)]
        },
        Some(test_accounts),
    );

    // account_user_1 was created before genesis and is not yet funded so fund it
    fund_account(&mut builder, account_user_1);

    let minting_account = account_user_1;
    let recipient: Key = minting_account.into();
    let mint_amount = U256::one();
    let id = U256::one();

    // account_user_1 is in minter list, request should succeed
    let mint_call = cep85_mint(
        &mut builder,
        &cep85_token,
        minting_account,
        recipient,
        id,
        mint_amount,
    );

    mint_call.expect_success().commit();

    // account_user_2 is not in minter list, request should fail
    let minting_account = *test_accounts.get(&ACCOUNT_USER_2).unwrap();
    let recipient: Key = minting_account.into();

    let failing_mint_call = cep85_mint(
        &mut builder,
        &cep85_token,
        minting_account,
        recipient,
        id,
        mint_amount,
    );

    failing_mint_call.expect_failure();

    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep85Error::InsufficientRights as u16,
        "should not allow to mint for non default admin account",
    );
}

#[test]
fn test_security_burner_rights() {
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
            BURNER_LIST => vec![Key::from(account_user_1)]
        },
        Some(test_accounts),
    );

    // account_user_1 was created before genesis and is not yet funded so fund it
    fund_account(&mut builder, account_user_1);

    // Set total supply to 2 fro the token to be minted

    let minting_account = *DEFAULT_ACCOUNT_ADDR;
    let recipient: Key = account_user_1.into();
    let total_supply = U256::from(2);
    let mint_amount = U256::from(2);
    let id = U256::one();

    let set_total_supply_of_call = cep85_set_total_supply_of(
        &mut builder,
        &cep85_token,
        minting_account,
        id,
        total_supply,
    );
    set_total_supply_of_call.expect_success().commit();

    let mint_call = cep85_mint(
        &mut builder,
        &cep85_token,
        minting_account,
        recipient,
        id,
        mint_amount,
    );

    mint_call.expect_success().commit();

    // account_user_2 is not in burner list, request should fail
    let burning_account = *test_accounts.get(&ACCOUNT_USER_2).unwrap();
    // owner is now last recipient
    let owner: Key = recipient;
    let burn_amount = U256::one();

    let failing_burn_call = cep85_burn(
        &mut builder,
        &cep85_token,
        burning_account,
        owner,
        id,
        burn_amount,
    );

    failing_burn_call.expect_failure();

    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep85Error::InsufficientRights as u16,
        "should not allow to mint for non default admin account",
    );

    // account_user_1 is in burner list, request should succeed
    let burning_account = *test_accounts.get(&ACCOUNT_USER_1).unwrap();

    let burn_call = cep85_burn(
        &mut builder,
        &cep85_token,
        burning_account,
        owner,
        id,
        burn_amount,
    );
    burn_call.expect_success().commit();

    // default address is in admin list, request should succeed
    let burning_account = *DEFAULT_ACCOUNT_ADDR;
    let owner: Key = burning_account.into();

    let burn_call = cep85_burn(
        &mut builder,
        &cep85_token,
        burning_account,
        owner,
        id,
        burn_amount,
    );
    burn_call.expect_success().commit();
}

#[test]
fn test_change_security() {
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
            ADMIN_LIST => vec![Key::from(account_user_1)],
        },
        None,
    );

    let minting_account = account_user_1;
    let recipient: Key = minting_account.into();
    let total_supply = U256::from(2);
    let mint_amount = U256::one();
    let id = U256::one();

    let set_total_supply_of_call = cep85_set_total_supply_of(
        &mut builder,
        &cep85_token,
        minting_account,
        id,
        total_supply,
    );

    set_total_supply_of_call.expect_success().commit();

    let mint_call = cep85_mint(
        &mut builder,
        &cep85_token,
        minting_account,
        recipient,
        id,
        mint_amount,
    );

    // account_user_1 is in admin list
    mint_call.expect_success().commit();

    let account_user_2 = *test_accounts.get(&ACCOUNT_USER_2).unwrap();
    let minting_account = account_user_2;
    let recipient: Key = account_user_2.into();

    let security_lists = SecurityLists {
        minter_list: Some(vec![Key::Account(account_user_2)]),
        burner_list: None,
        meta_list: None,
        admin_list: None,
        none_list: None,
    };

    let change_security =
        cep85_change_security(&mut builder, &cep85_token, account_user_1, security_lists);

    change_security.expect_success().commit();

    let mint_call = cep85_mint(
        &mut builder,
        &cep85_token,
        minting_account,
        recipient,
        id,
        mint_amount,
    );

    // account_user_2 is in minter list
    mint_call.expect_success().commit();

    let security_lists = SecurityLists {
        minter_list: None,
        burner_list: None,
        meta_list: None,
        admin_list: None,
        none_list: Some(vec![Key::Account(account_user_2)]),
    };

    let change_security =
        cep85_change_security(&mut builder, &cep85_token, account_user_1, security_lists);

    change_security.expect_success().commit();

    let failing_mint_call = cep85_mint(
        &mut builder,
        &cep85_token,
        minting_account,
        recipient,
        id,
        mint_amount,
    );

    // minting account is in none list now, so same request shoudl fail
    failing_mint_call.expect_failure();

    let error = builder.get_error().expect("must have error");

    assert_expected_error(
        error,
        Cep85Error::InsufficientRights as u16,
        "should not allow to mint for non default admin account",
    );
}
